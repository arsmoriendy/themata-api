macro_rules! export {
    ($name:ident) => {
        mod $name;
        pub use $name::$name;
    };
    ($($name:ident),+$(,)?) => {
        $(
            export!($name);
        )+
    };
}

export!(
    list,
    github_login,
    delete,
    read,
    create,
    update,
    authenticate,
    update_username,
    read_username,
    count,
);
