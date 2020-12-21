pub struct Params {
    pub filenames: Vec<String>,
    pub frequency: f64,
    pub spread: f64
}

pub fn process_params(input: Vec<String>) -> Params {
    let mut frequency = 0.1;    
    let mut spread = 3.0;
    let mut filenames = vec![];

    let mut iter = input.iter();
    iter.next();
    while let Some(param) = iter.next() {
        if param == "-f" || param == "--freq" {
            match iter.next() {
                Some(n) => frequency = n.parse::<f64>().unwrap(),
                None => panic!("Invalid frequency value")
            }
        } else if param == "-s" || param == "--spread" {
            match iter.next() {
                Some(n) => spread = n.parse::<f64>().unwrap(),
                None => panic!("Invalid spread value")
            }
        } else {
            filenames.push(param.to_string());
        }
    }
    return Params {
        filenames: filenames,
        frequency: frequency,
        spread: spread
    };
}

#[cfg(test)]
mod tests {
    macro_rules! vec_of_strings {
        ($($x:expr),*) => (vec![$($x.to_string()),*]);
    }
    
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_should_parse_filenames() {
        let input: Vec<String> = vec_of_strings!["", "filename-1.txt", "filename-2.html"];
        let params = process_params(input);
        assert_eq!(params.filenames.len(), 2);
        assert_eq!(params.filenames[0], "filename-1.txt");
        assert_eq!(params.filenames[1], "filename-2.html");
    }

    #[test]
    fn test_should_parse_frequency_parameter_short_param_name() {
        let input: Vec<String> = vec_of_strings!["", "-f", "2.0", "filename-1.txt"];
        let params = process_params(input);
        assert_eq!(params.filenames.len(), 1);
        assert_eq!(params.frequency, 2.0);
    }

    #[test]
    fn test_should_parse_frequency_parameter_long_param_name() {
        let input: Vec<String> = vec_of_strings!["", "--freq", "2.0", "filename-1.txt"];
        let params = process_params(input);
        assert_eq!(params.filenames.len(), 1);
        assert_eq!(params.frequency, 2.0);
    }

    #[test]
    fn test_should_parse_spread_parameter_short_param_name() {
        let input: Vec<String> = vec_of_strings!["", "-s", "2.0", "filename-1.txt"];
        let params = process_params(input);
        assert_eq!(params.filenames.len(), 1);
        assert_eq!(params.spread, 2.0);
    }

    #[test]
    fn test_should_parse_spread_parameter_long_param_name() {
        let input: Vec<String> = vec_of_strings!["", "--spread", "2.0", "filename-1.txt"];
        let params = process_params(input);
        assert_eq!(params.filenames.len(), 1);
        assert_eq!(params.spread, 2.0);
    }
}

