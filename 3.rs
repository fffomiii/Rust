//Удалите это, когда завершите работу над кодом
//#![allow(unused_variables, dead_code)]

fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut matrix2 = [[0; 3]; 3];
    for num in 0..3{
        for num2 in 0..3{
            matrix2[num][num2] = matrix[num2][num];
        }
    }
    matrix2
}

#[test]
fn test_transpose() {
    let matrix = [
        [101, 102, 103], //
        [201, 202, 203],
        [301, 302, 303],
    ];
    let transposed = transpose(matrix);
    assert_eq!(
        transposed,
        [
            [101, 201, 301], //
            [102, 202, 302],
            [103, 203, 303],
        ]
    );
}

fn main() {
    let matrix = [
        [101, 102, 103], //<-- комментарий заставляет rustfmt добавить новую строку
        [201, 202, 203],
        [301, 302, 303],
    ];

    println!(" матрица : {:#?}", matrix);
    let transposed = transpose(matrix);
    println!(" транспонированная матрица : {:#?}", transposed);
}
