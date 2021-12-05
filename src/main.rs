//Pinchuk Anna TI-81 | Variant 14


/*
 Task 1020
 Элемент матрицы называется седловой точкой, если он является одновременно наименьшим в своей строке
 и наибольшим в своем столбце. Дана действительная матрица размера 5x6.
 Выяснить, имеются ли седловые точки в этой матрице, и если имеются, то указать индексы одной из них
*/

fn task1020(a: &Vec<Vec<f64>>) -> Vec<Vec<usize>> {
    assert_eq!(a.len(), 5);
    assert_eq!(a[0].len(), 6);

    let mut row_mins = vec![f64::INFINITY; 5];
    let mut column_maxes = vec![f64::NEG_INFINITY; 6];
    let mut result = vec![];

    for row in 0..5 {
        for column in 0..6 {
            if a[row][column] < row_mins[row] {
                row_mins[row] = a[row][column];
            }

            if a[row][column] > column_maxes[column] {
                column_maxes[column] = a[row][column];
            }
        }
    }

    for row in 0..5 {
        for column in 0..6 {
            if a[row][column] == row_mins[row] && a[row][column] == column_maxes[column] {
                // println!("{} ({}; {})", a[row][column], row, column);
                result.push(vec![row, column]);
            }
        }
    }

    result
}

#[cfg(test)]
mod tests1020 {
    use super::*;

    #[test]
    fn simple() {
        let a = vec![
            vec![1., 2., 3., 4., 5., 6.],
            vec![2., 8., 9., 10., 11., 12.],
            vec![3., 14., 15., 16., 17., 18.],
            vec![2., 20., 21., 22., 23., 6.],
            vec![1., 2., 3., 4., 5., 6.],
        ];

        let expected = vec![
            vec![2, 0],
        ];

        let result = task1020(&a);

        assert_eq!(expected, result);
    }
}

/*
 Task 577
 Даны натуральные числа а,...,а_10. Предположим, что имеются 10 видов монет достоинством а,...,а_10.
 Обозначим через b_k число способов, которыми можно выплатить сумму k, т.е. b_k — это число решений
 уравнения а_1*х_1 + ... + а_10*х_10 = k, где x_i может принимать целые неотрицательные значения.
 Получить b_0,...,b_20
*/

fn task577(a: &[usize; 10], k_arr: &[usize]) -> Vec<usize> {
    let mut result = vec![];

    for x in a {
        print!("{:4} ", x);
    }
    println!("\n{}", "-".repeat(a.len() * 5));

    for &k in k_arr {
        let b: Vec<usize> = a.iter().map(|&x| k / x).collect();//прораховуємо макс кількість монет
        let mut c = vec![0; 10]; //поточна кількість кожної монети
        let mut n = 0;
        let mut q = false;

        loop {
            let mut s = 0;

            for i in 0..a.len() {
                s += a[i] * c[i]; //до суми додаються ціни поточних кількостей монет
            }

            if s == k { //якщо поточна сума досягла цільової суми
                n += 1;
                for c_i in 0..10 {
                    print!("{:4} ", c[c_i]);
                }
                println!();
            }

            for i in (0..10).rev() {
                q = c[i] == b[i]; //чи досягнуто макс значення поточні кількості i-ї монети

                if q {
                    let mut j = i;
                    while j < 10 {
                        c[j] = 0; //обнуляємо поточні кількості від i-ї монети до останньої
                        j += 1;
                    }
                } else {
                    c[i] += 1; //інкрементуємо кількість і-ї монети
                    break;
                }
            }

            if q { break; } //якщо остання монета включена в суму максимальною кількістю
        }

        result.push(n);

        println!("b = {}", n); //друкуємо кількість наборів
    }

    result
}

#[cfg(test)]
mod tests577 {
    use super::*;

    #[test]
    fn simple() {
        let a = [1, 2, 5, 10, 25, 50, 100, 200, 500, 1000];
        let k = [5];

        let expected: Vec<usize> = vec![4];

        let result = task577(&a, &k);

        assert_eq!(expected, result);
    }
}


/*
 Task 692(г)
 Дана действительная квадратная матрица порядка л. Найти наибольшее из значений элементов,
 расположенных в заштрихованной части матрицы
*/

fn task692g(a: &Vec<Vec<f64>>) -> f64 {
    let n = a.len();
    let center: usize = n / 2;
    let mut max = 0.;

    let mut k = 0;

    //println!("{:?}", (center..n));

    for i in (center..n).rev() {
        for j in k..n - k {
            println!("a[{}][{}]: {}", i, j, a[i][j]);

            if a[i][j] > max{
                max = a[i][j];
            }
        }
        k += 1;
        //println!("k={}", k);
    }
    println!();

    max
}


#[cfg(test)]
mod tests692g {
    use super::*;

    #[test]
    fn matrix3x3() {
        let a = vec![
            vec![3., 2., 6.],
            vec![5., 8., 9.],
            vec![11., 3., 10.],
        ];

        let result = task692g(&a);

        assert_eq!(11., result);
    }

    #[test]
    fn matrix4x4() {
        let a = vec![
            vec![11., 25., 12., 4.],
            vec![5., 10., 14., 11.],
            vec![12., 27., 11., 13.],
            vec![66., 18., 45., 19.],
        ];

        let result = task692g(&a);

        assert_eq!(66., result);
    }
}

fn main() {

}
