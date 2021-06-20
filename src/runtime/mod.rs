cfg_if::cfg_if! {
    if #[cfg(feature = "kusama")] {
        pub mod kusama;
        pub use kusama::Runtime;
    } else if #[cfg(feature = "node_template")]  {
        pub mod node_template;
        pub use node_template::Runtime;
    } else {
        pub mod node_template;
        pub use node_template::Runtime;
    }
}