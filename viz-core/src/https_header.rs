macro_rules! standard_headers {
    ($(($struct_name:ident, $const_name:ident, $bytes:expr));* $(;)?) => {
        $(
            // 定义结构体
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct $struct_name;

            impl $struct_name {
                // 提供获取头部名称的方法
                pub const fn name() -> &'static [u8] {
                    $bytes
                }
                
                // 创建一个带有值的头部
                pub fn with_value<T: Into<String>>(value: T) -> (Vec<u8>, String) {
                    (Self::name().to_vec(), value.into())
                }
            }

            // 定义常量（字节字符串）
            pub const $const_name: &[u8] = $bytes;
        )*
    };
}

standard_headers! {
    (Accept, ACCEPT, b"accept");
    (ContentType, CONTENT_TYPE, b"content-type");
    (Authorization, AUTHORIZATION, b"authorization");
}
