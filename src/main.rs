fn main() {
    //调用sum函数 参数传入一个&[u32]数据类型 返回一个option_u32 用res变量接收
    let res = sum(&[4294967295,100,50]);
    //打印res
    println!("{:#?}",res)
}
//声明和定义sum函数 传入&[u32]引用类型数组 返回一个option_u32
fn sum(v:&[u32])->Option<u32>{
    //定义一个累计变量res
    let mut res:u32 = 0;
    //遍历[u32]然后把值进行叠加
    for i in v{
        //判断有否溢出 如有溢出则返回None
        if res > res.wrapping_add(*i){
            return None
        }
        //没有溢出则求和
        res = res.wrapping_add(*i);
    }
    //返回最终结果
    Some(res)
}