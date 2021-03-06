# LR

1. 首先是``自底向上``分析过程: ``为一个输入串构造语法分析树的过程``
2. LR(k)分析技术: 
   1. L:从左向右
   2. R: 反向构造一个最右推导序列
   3. k: 做出语法分析决定时向前看k个输入符号

当然在实践中我们只考虑``k=0或k=1``的情况

为啥要用LR语法分析器呢? LL不香吗?
1. 几乎所有的程序语言, 只要能写出改语言的``上下文无关文法``, 就可以构造出相应的LR语法分析器. 
2. LR无回溯, 很高效
3. LR(k)分析能力强于LL(k)
4. 总结一下. LR分析的优点
   1. 高性能
   2. 高能力
   3. 使用范围广
当然LR的缺点也是有的.
是时候拿出万年老二``if c1 then  if c2 then e2 else e3``来抬杠了.
   但我们可以稍微改写成``if c1 { if c2 {e2 } else {e3}}``.
还有手写LR分析是火葬场. 就比如我写的一个小[demo](https://github.com/wangbyby/compiler/blob/master/src/frontend/parser.rs). 

既然LR是``移近归约``分析器. 那何时移近, 何时归约?

移近&归约
---

我们通过维护一些``状态``, 来指导我们做出移近或归约的决定.
状态?
我们通过$A \rightarrow X.YZ$中``点``的位置来表示当下状态. **点左边的是可以由$X$推导得到的串, 点右边是接下来想看到一个能从$YZ$推导得到的串.**

用状态的思想我们可以得出LR(0)自动机, 如图所示的例子...

<!-- 图 -->
为啥可以使用LR(0)自动机来做出``移近/归约``决定?
因为LR(0)自动机可以刻画出可能出现在分析器栈中的文法符号串.
即``LR(0)自动机能识别可行前缀``

对于一个可行前缀, 但前面有多条路怎么选?
可以查看下一个输入符号来解决.

这样就得到了``SLR(1)``
SLR(1)与LR(0)的不同在于: if$A \rightarrow \alpha. $ for b in FOLLOW(A): Action[i,b]=Reduce$A \rightarrow \alpha. $
FOLLOW(A)一把梭就可以解决问题了吗?
>以下文法SLR(1)就不能识别
$$S\rightarrow L=R | R $$
$$L \rightarrow *R |id$$
$$R \rightarrow L$$
FOLLOW(L) = FOLLOW(R) = {#,=}
对于$L.=R$来说, 下一步动作有$归约R \rightarrow L.$或者$移近S \rightarrow L.=R$. 然而我们没有以$R=...$开头的句型.

所以我们需要更为精细的操作. 

LR(1)出现了!
只有在形如$[A \rightarrow \alpha. ,a]$的项且下一个输入符号为``a``的情况下, 我们才会按照$A \rightarrow \alpha$进行归约.

那如何计算下一个符号``a``呢?
<!-- 图2 -->

LR(1)的问题呢?
重复项太多了. ~~LALR(1)也不少~~

我们用LALR(1)来代替LR(1). 虽然分辨率降低了,但是对于常见语言还是🆗的.

欲听后事如何, 请听下回分解.