# 转换执行边界

## 问题

`ConversionSession` 持有累计策略和资源记账。把它直接传给每个
`DataConversionTarget` 会让下游目标获得原始操作：它们可以按任意类型对记账、重复准入，
也可能遗漏准入。这些错误可以通过编译，却很难从调用方审计。

## 决策

公开的目标扩展边界改为 `AdmittedConversion<'_, '_, '_>`。`DataConverter` 只为顶层条目及其
输入准入一次，然后创建绑定运行时来源类型和请求目标类型的 context，包装为
`AdmittedConversion` 后再调用目标。`AdmittedScalarItem` 在消耗一次性 token 后遵循同一规则。

下游目标收到的是公开封装 `AdmittedConversion`，而不是内部
`ConversionContext`。封装公开 `source()`、`from_type()`、`to_type()`、
`unsupported()`、消费式同描述符 `convert()`，启用 `json` 时的
`decode_json()`、`account_json_value()`、`encode_json()`，以及事务式
`write_string()`。这些操作仍使用框架选定的策略和同一累计会话预算。内部
context 还提供直接访问 `policy()`、`limits()` 和 JSON `decode_json_seed()` 的方法；
下游目标不能调用这些 context 方法。

`ConversionSession` 仍公开构造、复用、标量条目准入、配置和累计用量观察。原始字节和
结构记账原语改为 crate 私有。

标量集合适配器先调用 `admit_scalar_string_source`，为整个输入记账，并返回
`AdmittedScalarSource`。该对象借用 session，并从已记账的原始文本中惰性地产生一次性
`AdmittedScalarItem` token。它保留过滤前的原始来源索引，且不会分配条目字符串。拆分或准入
失败会耗尽该来源；消费者提前结束时，尚未访问的尾部不会被扫描。没有公开入口可以脱离
来源字节记账，单独准入任意替代条目来源。

`StructuredConversionLimits::value()` 是可复用的值限额模板：其中字符串字节限额与
`max_text_bytes` 相同，直接使用模板的消费者会执行该限额。内置转换会话只选取模板的结构限额，
并叠加累计操作预算，不应用该字符串字节限额。`BooleanConversionPolicyBuilder` 会保留尚未验证的
字段，直到 `build()` 时统一验证最终组合，因此构建结果不依赖 setter 顺序。

公开的 `AdmittedConversion::decode_json()` helper 会在解码前，按自定义目标实际传入的字节长度
检查 `max_text_bytes`，其中包括空白。它既不规范化，也不会再次准入来源。检查拒绝时，已扣除的
条目和来源字节仍保留扣减，且不会新增结构或输出用量。内置文本转 JSON 路径仍会在解码前检查
规范化后的来源文本。

JSON 解码失败时，`DataConversionError::json_diagnostic()` 只保留本 crate 定义的错误类别及可选行列，
不包含输入值、对象键或后端错误文本。位置对应交给解码器的文本（可能已规范化）；语法错误的列按
Unicode 字符计数，目标反序列化错误的列按 UTF-8 字节计数。缺失的位置保持为 `None`。预算错误仍
保持 `LimitExceeded` 分类且不附带 JSON 诊断；诊断也不改变既有错误相等性或错误来源链。

原始 `ScalarItems` 迭代器在拒绝空条目后仍可继续，并按原始来源索引报告错误；超过保留条目数时只
报错一次，随后耗尽。`AdmittedScalarSource` 则在拆分或条目准入错误（包括拒绝空条目）后耗尽。
已经准入条目后发生的目标转换失败属于后续独立操作。

## 不变量

1. 顶层 converter 调用在目标执行前为条目和输入记账。
2. 委托不会再次进行顶层准入。
3. 每个 context 预算错误都携带当前来源和目标类型。
4. String 渲染或输出准入失败不会提交输出字节。
5. 标量条目 token 只能消耗一次，并且获得已绑定的 context。
6. 每个标量条目 token 都源自已按字节准入的来源，且条目不能超出其对 session 的独占借用。
7. 只有被委托目标的 `DataType` 与 context 绑定的外层目标相同时才允许委托。描述符不同时，
   框架会在调用被委托目标前返回 `Unsupported`；错误保留原始来源类型和外层目标类型。此前已
   扣除的条目与成功扣除的输入用量不回滚，本次拒绝不新增扣减，也不产生输出。
8. 内置目标实现在 trait 直接入口也执行相同的描述符检查，并在消耗准入转换或执行任何转换工作前
   完成检查。因此，直接调用 trait 和普通委托都受此保护。下游自定义实现仍由其实现者负责；框架
   不会隔离任意下游代码。

## 兼容性

这是有意做出的破坏性 API 变更：下游 `DataConversionTarget` 实现必须将参数从
`&mut ConversionSession` 改为 `AdmittedConversion`，并调用其转换方法。这一变更移除一种
能力，而不是再增加一套并行 API，使扩展模型保持单一且可审计。

同 `DataType` 委托约束也会收紧行为：此前可能成功的跨描述符委托现在可能返回
`Unsupported`。如果 newtype 只是增加领域约束，应沿用被包装类型的描述符，完成委托后再检查约束。
语义确实不同时，可通过 `source()` 实现检查和映射，并使用现有预算辅助方法处理输出；不要新建
session 来绕过累计限制。

## 运行时描述与能力发现

`DataType` 自身负责稳定名称和不依赖 feature 的语义分类。调用方直接使用
`DataType::as_str` 和 `DataType::category`。

转换路径拓扑只保留一份内部规范矩阵。公开的 `ConversionCapabilities` 门面先依据当前
Cargo features 过滤来源和目标的可用性，再查询该矩阵。这样既保留公开的 feature 能力
发现，又避免文档、能力查询与转换器分派关系各自演化出并行策略表。
