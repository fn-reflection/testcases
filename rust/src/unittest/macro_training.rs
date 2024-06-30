// original: https://gist.github.com/jonhoo/ec57882a976a2d2a92b3138ea25cd45a
// https://www.youtube.com/watch?v=q6paRBbLgNw

macro_rules! macro_allows_loose_pattern {
    ($arg1:ty, $arg2: expr, $arg3: path) => {};
    ($arg1:ty => $arg2: expr; $arg3: path) => {};
}

macro_rules! type_alias {
    ($arg1:ty => $Arg2: ident) => {
        type Args2 = $arg1;
    };
    ($arg1:ty) => {
        type MyU32 = $arg1;
    };
}

// マクロは衛生的である(以下のxはマクロ外から参照できない)
macro_rules! let_vars {
    () => {
        let x = 42;
    };
}

macro_rules! increment {
    ($x:ident) => {
        $x += 1;
    };
}

#[macro_export]
macro_rules! vector1 {
    () => {
        Vec::new();
    };
    // 外側の{}はmacro_rulesの要請
    // 内側の{}はブロックを作成するRustの基本構文、文を式に変える。
    // これがないとlet mut v = let mut vsのようにマクロ展開されるが、それは構文エラーになる。
    ($element:expr) => {{
        let mut vs = Vec::new();
        vs.push($element);
        vs
    }};
}

#[macro_export]
macro_rules! vector {
    ($($element:expr),* $(,)?) => {{
        #[allow(unused_mut)]
        let mut vs = Vec::with_capacity($crate::count![@COUNT; $($element),*]);
        $(vs.push($element);)*
        vs
    }};
    ($element:expr;$count:expr) => {{
        let count = $count;
        let mut vs = Vec::with_capacity(count);
        vs.resize(count, $element);
        // vs.extend(std::iter::repeat($element).take(count));
        vs
    }};
}

#[macro_export]
macro_rules! avec {
    ($($element:expr),*) => {{
        // check that count is const
        const C: usize = $crate::count![@COUNT; $($element),*];

        #[allow(unused_mut)]
        let mut vs = Vec::with_capacity(C);
        $(vs.push($element);)*
        vs
    }};
    ($($element:expr,)*) => {{
        $crate::avec![$($element),*]
    }};
    ($element:expr; $count:expr) => {{
        let mut vs = Vec::new();
        vs.resize($count, $element);
        vs
    }};

}


#[macro_export]
#[doc(hidden)]
macro_rules! count {
    (@COUNT; $($element:expr),*) => {
        // $elementの要素数に応じて[(), (), ...]のようなユニット配列に展開される
        // $elementの要素数をlen()を通じてコンパイル時に計数できる
        <[()]>::len(&[$($crate::count![@SUBST; $element]),*])
    };
    (@SUBST; $_element:expr) => { () };
}


// unit testに対するマクロの展開方法
// cargo expand --lib unittest::macro_training --tests

// このファイルのテストの実行方法
// cargo test unittest::macro_training  

#[test]
fn macro_test_ok() {
    macro_allows_loose_pattern![u32, x.foo(), std::path];
    macro_allows_loose_pattern![u32=> x.foo(); std::path];
    type_alias! { u32 => MyU32 };
    type_alias! { u32 };
    let mut x = 42;
    let_vars!();
    increment!(x);
    let v = vector![] as Vec<u32>;
}

#[test]
fn vector_empty_ok() {
    let v = vector![] as Vec<u32>;
    assert!(v.is_empty());
}

#[test]
fn vector_some_ok() {
    let v = vector1![42];
    assert_eq!(v.len(), 1);
    assert_eq!(v[0], 42);
}

#[test]
fn vector_some_ok2() {
    let v = vector![42, 43,];
    assert_eq!(v.len(), 2);
    assert_eq!(v[0], 42);
    assert_eq!(v[1], 43);
}


#[test]
fn vector_some_ok3() {
    let v = vector![42; 3];
    assert_eq!(v.len(), 3);
    assert_eq!(v[0], 42);
    assert_eq!(v[1], 42);
    assert_eq!(v[2], 42);
}

/// compileが失敗することを示すテスト
/// ```compile_fail
/// let x: Vec<u32> = rust_studies::vector![42; "foo"];
/// ```
struct CompileFailTest;