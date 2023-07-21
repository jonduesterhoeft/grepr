use grepr;

#[test]
fn test_search_line_case_noinvert_good() {
    let query = "this is a test.".to_string();
    let path = PathBuf::new();
    let contents = "this is a test.\nthis is another test!";
    let ignore_case = false;
    let invert_match = false;
    let word = false;
    let line = true;

    let args = CommandArgs { 
        query, 
        path,
        ignore_case,
        invert_match,
        word,
        line 
    };

    let mut search = Search::new(&contents);
    let _ = search.find(&args);

    assert_eq!(search.results[0].1, "this is a test.")
}



#[test]
fn test_search_line_case_noinvert_bad() {
    let query = "this is a test".to_string();
    let path = PathBuf::new();
    let contents = "this is a test.\nthis is another test!";
    let ignore_case = false;
    let invert_match = false;
    let word = false;
    let line = true;

    let args = CommandArgs { 
        query, 
        path,
        ignore_case,
        invert_match,
        word,
        line 
    };

    let mut search = Search::new(&contents);
    let _ = search.find(&args);

    assert_eq!(search.results.len(), 0)
}

#[test]
fn test_search_line_nocase_noinvert_good() {
    let query = "THIS is a test.".to_string();
    let path = PathBuf::new();
    let contents = "this is a test.\nthis is another test!";
    let ignore_case = true;
    let invert_match = false;
    let word = false;
    let line = true;

    let args = CommandArgs { 
        query, 
        path,
        ignore_case,
        invert_match,
        word,
        line 
    };

    let mut search = Search::new(&contents);
    let _ = search.find(&args);

    assert_eq!(search.results[0].1, "this is a test.")
}



#[test]
fn test_search_line_nocase_noinvert_bad() {
    let query = "THIS is a test".to_string();
    let path = PathBuf::new();
    let contents = "this is a test.\nthis is another test!";
    let ignore_case = true;
    let invert_match = false;
    let word = false;
    let line = true;

    let args = CommandArgs { 
        query, 
        path,
        ignore_case,
        invert_match,
        word,
        line 
    };

    let mut search = Search::new(&contents);
    let _ = search.find(&args);

    assert_eq!(search.results.len(), 0)
}

#[test]
fn test_search_line_nocase_invert_good() {
    let query = "THIS is a test.".to_string();
    let path = PathBuf::new();
    let contents = "this is a test.\nthis is another test!";
    let ignore_case = true;
    let invert_match = true;
    let word = false;
    let line = true;

    let args = CommandArgs { 
        query, 
        path,
        ignore_case,
        invert_match,
        word,
        line 
    };

    let mut search = Search::new(&contents);
    let _ = search.find(&args);

    assert_eq!(search.results[0].1, "this is another test!")
}



#[test]
fn test_search_line_nocase_invert_bad() {
    let query = "THIS is a test".to_string();
    let path = PathBuf::new();
    let contents = "this is a test.\nthis is another test!";
    let ignore_case = true;
    let invert_match = true;
    let word = false;
    let line = true;

    let args = CommandArgs { 
        query, 
        path,
        ignore_case,
        invert_match,
        word,
        line 
    };

    let mut search = Search::new(&contents);
    let _ = search.find(&args);

    assert_eq!(search.results.len(), 2)
}

#[test]
fn test_search_word_case_noinvert_good() {
    let query = "another".to_string();
    let path = PathBuf::new();
    let contents = "this is a test.\nthis is another test!";
    let ignore_case = false;
    let invert_match = false;
    let word = true;
    let line = false;

    let args = CommandArgs { 
        query, 
        path,
        ignore_case,
        invert_match,
        word,
        line 
    };

    let mut search = Search::new(&contents);
    let _ = search.find(&args);

    assert_eq!(search.results[0].1, "this is another test!")
}



#[test]
fn test_search_word_case_noinvert_bad() {
    let query = "nothing".to_string();
    let path = PathBuf::new();
    let contents = "this is a test.\nthis is another test!";
    let ignore_case = false;
    let invert_match = false;
    let word = true;
    let line = false;

    let args = CommandArgs { 
        query, 
        path,
        ignore_case,
        invert_match,
        word,
        line 
    };

    let mut search = Search::new(&contents);
    let _ = search.find(&args);

    assert_eq!(search.results.len(), 0)
}

#[test]
fn test_search_word_nocase_noinvert_good() {
    let query = "ANOTHER".to_string();
    let path = PathBuf::new();
    let contents = "this is a test.\nthis is another test!";
    let ignore_case = true;
    let invert_match = false;
    let word = true;
    let line = false;

    let args = CommandArgs { 
        query, 
        path,
        ignore_case,
        invert_match,
        word,
        line 
    };

    let mut search = Search::new(&contents);
    let _ = search.find(&args);

    assert_eq!(search.results[0].1, "this is another test!")
}



#[test]
fn test_search_word_nocase_noinvert_bad() {
    let query = "NOTHING".to_string();
    let path = PathBuf::new();
    let contents = "this is a test.\nthis is another test!";
    let ignore_case = true;
    let invert_match = false;
    let word = true;
    let line = false;

    let args = CommandArgs { 
        query, 
        path,
        ignore_case,
        invert_match,
        word,
        line 
    };

    let mut search = Search::new(&contents);
    let _ = search.find(&args);

    assert_eq!(search.results.len(), 0)
}

#[test]
fn test_search_word_nocase_invert_good() {
    let query = "another".to_string();
    let path = PathBuf::new();
    let contents = "this is a test.\nthis is another test!";
    let ignore_case = true;
    let invert_match = true;
    let word = true;
    let line = false;

    let args = CommandArgs { 
        query, 
        path,
        ignore_case,
        invert_match,
        word,
        line 
    };

    let mut search = Search::new(&contents);
    let _ = search.find(&args);

    assert_eq!(search.results[0].1, "this is a test.")
}



#[test]
fn test_search_word_nocase_invert_bad() {
    let query = "nothing".to_string();
    let path = PathBuf::new();
    let contents = "this is a test.\nthis is another test!";
    let ignore_case = true;
    let invert_match = true;
    let word = true;
    let line = false;

    let args = CommandArgs { 
        query, 
        path,
        ignore_case,
        invert_match,
        word,
        line 
    };

    let mut search = Search::new(&contents);
    let _ = search.find(&args);

    assert_eq!(search.results.len(), 2)
}



#[test]
fn test_search_partial_case_noinvert_good() {
    let query = "ano".to_string();
    let path = PathBuf::new();
    let contents = "this is a test.\nthis is another test!";
    let ignore_case = false;
    let invert_match = false;
    let word = false;
    let line = false;

    let args = CommandArgs { 
        query, 
        path,
        ignore_case,
        invert_match,
        word,
        line 
    };

    let mut search = Search::new(&contents);
    let _ = search.find(&args);

    assert_eq!(search.results[0].1, "this is another test!")
}



#[test]
fn test_search_partial_case_noinvert_bad() {
    let query = "nothing".to_string();
    let path = PathBuf::new();
    let contents = "this is a test.\nthis is another test!";
    let ignore_case = false;
    let invert_match = false;
    let word = false;
    let line = false;

    let args = CommandArgs { 
        query, 
        path,
        ignore_case,
        invert_match,
        word,
        line 
    };

    let mut search = Search::new(&contents);
    let _ = search.find(&args);

    assert_eq!(search.results.len(), 0)
}

#[test]
fn test_search_partial_nocase_noinvert_good() {
    let query = "ANO".to_string();
    let path = PathBuf::new();
    let contents = "this is a test.\nthis is another test!";
    let ignore_case = true;
    let invert_match = false;
    let word = false;
    let line = false;

    let args = CommandArgs { 
        query, 
        path,
        ignore_case,
        invert_match,
        word,
        line 
    };

    let mut search = Search::new(&contents);
    let _ = search.find(&args);

    assert_eq!(search.results[0].1, "this is another test!")
}



#[test]
fn test_search_partial_nocase_noinvert_bad() {
    let query = "NOTHING".to_string();
    let path = PathBuf::new();
    let contents = "this is a test.\nthis is another test!";
    let ignore_case = true;
    let invert_match = false;
    let word = false;
    let line = false;

    let args = CommandArgs { 
        query, 
        path,
        ignore_case,
        invert_match,
        word,
        line 
    };

    let mut search = Search::new(&contents);
    let _ = search.find(&args);

    assert_eq!(search.results.len(), 0)
}

#[test]
fn test_search_partial_nocase_invert_good() {
    let query = "ano".to_string();
    let path = PathBuf::new();
    let contents = "this is a test.\nthis is another test!";
    let ignore_case = true;
    let invert_match = true;
    let word = false;
    let line = false;

    let args = CommandArgs { 
        query, 
        path,
        ignore_case,
        invert_match,
        word,
        line 
    };

    let mut search = Search::new(&contents);
    let _ = search.find(&args);

    assert_eq!(search.results[0].1, "this is a test.")
}



#[test]
fn test_search_partial_nocase_invert_bad() {
    let query = "nothing".to_string();
    let path = PathBuf::new();
    let contents = "this is a test.\nthis is another test!";
    let ignore_case = true;
    let invert_match = true;
    let word = false;
    let line = false;

    let args = CommandArgs { 
        query, 
        path,
        ignore_case,
        invert_match,
        word,
        line 
    };

    let mut search = Search::new(&contents);
    let _ = search.find(&args);

    assert_eq!(search.results.len(), 2)
}

