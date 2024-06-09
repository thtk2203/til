#[allow(dead_code)]
fn result_vec_except_at(index: u32, count: u32) -> Result<Vec<u32>, String> {
    let mut indexes = vec![];

    (0..count).map(|i| {
        indexes.push(i.to_string());

        match i {
            _matched if i == index => Err(format!("{}", indexes.join(","))),
            _ => Ok(i),
        }
    })
    .collect::<Result<Vec<_>, _>>()
}


#[test]
fn ok_map_with_result_vec() {
    let actual = result_vec_except_at(3, 3);
    assert!(actual.is_ok());
}

#[test]
fn err_map_with_result_vec() {
    let actual = result_vec_except_at(2, 4);
    assert!(actual.is_err());
    assert_eq!(actual.unwrap_err(), "0,1,2");
}

#[allow(dead_code)]
fn vec_result_except_at(index: u32, count: u32) -> Vec<Result<u32, String>> {
    let mut indexes = vec![];

    (0..count).map(|i| {
        indexes.push(i.to_string());

        match i {
            _matched if i == index => Err(format!("{}", indexes.join(","))),
            _ => Ok(i),
        }
    })
    .collect::<Vec<Result<_, _>>>()
}

#[test]
fn ok_map_with_vec_result() {
    let actual = vec_result_except_at(3, 3);
    assert!(actual.iter().all(|r| r.is_ok()));
}

#[test]
fn err_map_with_vec_result() {
    let mut actual = vec_result_except_at(2, 4).into_iter();
    assert_eq!(actual.next().unwrap(), Ok(0u32));
    assert_eq!(actual.next().unwrap(), Ok(1u32));
    assert_eq!(actual.next().unwrap(), Err("0,1,2".to_string()));
    assert_eq!(actual.next().unwrap(), Ok(3u32));
}