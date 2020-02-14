#[cfg(test)]
mod calc_avg {
    use crate::*;

    #[test]
    fn returns_avg() {
        let data = vec![10, 15, 5];
        let result = calc_avg(&data);
        assert_eq!(result, 10);
    }

    #[test]
    fn returns_0_when_empty() {
        let data = vec![];
        let result = calc_avg(&data);
        assert_eq!(result, 0);
    }
}

#[cfg(test)]
mod median_spec {
    use crate::*;

    #[test]
    fn returns_median() {
        let data = vec![9, 15, 5];
        let result = calc_median(&data);
        assert_eq!(result, 9);
    }

    #[test]
    fn returns_0_when_empty() {
        let data = vec![];
        let result = calc_median(&data);
        assert_eq!(result, 0);
    }
}

#[cfg(test)]
mod calc_mode {
    use crate::*;

    #[test]
    fn returns_mode() {
        let data = vec![40, 1, 300, 4, 1, 3, 3, 3];
        let result = calc_mode(&data);
        assert_eq!(result, 3);
    }

    #[test]
    fn returns_0_when_empty() {
        let data = vec![];
        let result = calc_mode(&data);
        assert_eq!(result, 0);
    }
}

#[cfg(test)]
mod batch_calc {
    use crate::*;
    #[test]
    fn returns_batch_calc_result() {
        let data = vec![40, 1, 2, 2, 5, 300, 4, 1, 3, 3, 3];
        let result = batch_calc(&data);
        println!("{:?}", result);
        assert_eq!(result.median, 3);
        assert_eq!(result.mode, 3);
        assert_eq!(result.avg, 33);
    }
}
