# 转换执行边界

## 问题

`ConversionSession` 持有累计策略和资源记账。把它直接传给每个
`DataConversionTarget` 会让下游目标获得原始操作：它们可以按任意类型对记账、重复准入，
也可能遗漏准入。这些错误可以通过编译，却很难从调用方审计。

## 决策

公开的目标扩展边界改为 `AdmittedConversion<'_, '_, '_>`。`DataConverter` 只为顶层条目及其
输入准入一次，然后创建绑定运行时来源类型和请求目标类型的 context，包装为
`AdmittedConversion` 后再调用目标。`AdmittedScalarItem` 在消耗一次性 token 后遵循同一规则。

context 只暴露当前转换中有意义的操作：

- 借用和拥有形式的嵌套委托；
- 策略、限制和已绑定类型的访问器；
- 启用 `json` 时的 JSON 解码、seed 解码、值记账和编码；
- 事务式 String 渲染。

`ConversionSession` 仍公开构造、复用、标量条目准入、配置和累计用量观察。原始字节和
结构记账原语改为 crate 私有。

标量集合适配器先调用 `admit_scalar_string_source`，为整个输入记账，并返回
`AdmittedScalarSource`。该对象借用 session，并从已记账的原始文本中惰性地产生一次性
`AdmittedScalarItem` token。它保留过滤前的原始来源索引，且不会分配条目字符串。拆分或准入
失败会耗尽该来源；消费者提前结束时，尚未访问的尾部不会被扫描。没有公开入口可以脱离
来源字节记账，单独准入任意替代条目来源。

## 不变量

1. 顶层 converter 调用在目标执行前为条目和输入记账。
2. 委托不会再次进行顶层准入。
3. 每个 context 预算错误都携带当前来源和目标类型。
4. String 渲染或输出准入失败不会提交输出字节。
5. 标量条目 token 只能消耗一次，并且获得已绑定的 context。
6. 每个标量条目 token 都源自已按字节准入的来源，且条目不能超出其对 session 的独占借用。

## 兼容性

这是有意做出的破坏性 API 变更：下游 `DataConversionTarget` 实现必须将参数从
`&mut ConversionSession` 改为 `AdmittedConversion`，并调用其转换方法。这一变更移除一种
能力，而不是再增加一套并行 API，使扩展模型保持单一且可审计。

## 运行时描述与能力发现

`DataType` 自身负责稳定名称和不依赖 feature 的语义分类。调用方直接使用
`DataType::as_str` 和 `DataType::category`。

转换路径拓扑只保留一份内部规范矩阵。公开的 `ConversionCapabilities` 门面先依据当前
Cargo features 过滤来源和目标的可用性，再查询该矩阵。这样既保留公开的 feature 能力
发现，又避免文档、能力查询与转换器分派关系各自演化出并行策略表。
