macro_rules! define_actions {
    (
        $(
            $kind:ident => $type:ty
        ),* $(,)?
    ) => {
        #[derive(Clone)]
        pub enum ActionKind {
            $( $kind($type) ),*
        }

        impl Action for ActionKind {
            fn apply(&self, path: &Path) {
                match self {
                    $( ActionKind::$kind(action) => action.apply(path) ),*
                }
            }

        }
    };
}
