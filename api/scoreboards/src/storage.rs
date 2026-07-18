use crate::*;
use erased_set::ErasedSendSet;
use spin::Mutex;

pub static STORE: Mutex<Option<ErasedSendSet>> = Mutex::new(None);


pub fn init_store() {
    let mut store = STORE.lock();
    if store.is_none() {
        *store = Some(ErasedSendSet::new());
    }
}

pub fn clean_store() {
    let mut store = STORE.lock();
    if let Some(true) = store.as_mut().map(|s| s.is_empty()) {
        *store = None;
        println!("store cleaned up");
    }
}


pub unsafe extern "C" fn proxy_boards<F: 'static + Send + FnMut(ScoresResult<Boards>)>(boards: *mut PDBoardsList,
                                                                                       error: *const c_char) {
    let res = if boards.is_null() {
        Err(Error::from_ptr(error).expect("unable read err"))
    } else {
        if !error.is_null() {
            let err = Error::from_ptr(error).expect("unable read err");
            sys::println!("Err: {err}");
        }

        Ok(Boards(boards))
    };

    let f = STORE.lock().as_mut().map(|store| store.remove::<F>()).flatten();
    f.map(|mut f| f(res)).or_else(|| panic!("missed callback"));

    clean_store();
}


pub unsafe extern "C" fn proxy_scores<F: 'static + Send + FnMut(ScoresResult<Scores>)>(scores: *mut PDScoresList,
                                                                                       error_message: *const c_char)
{
    let res = if scores.is_null() {
        Err(Error::from_ptr(error_message).expect("unable read err"))
    } else {
        if !error_message.is_null() {
            let err = Error::from_ptr(error_message).expect("unable read err");
            sys::println!("Err: {err}");
        }

        Ok(Scores(scores))
    };

    let f = STORE.lock().as_mut().map(|store| store.remove::<F>()).flatten();
    f.map(|mut f| f(res)).or_else(|| panic!("missed callback"));

    clean_store();
}

pub unsafe extern "C" fn proxy_score<F: 'static + Send + FnMut(ScoresResult<ScoreRef>)>(score: *mut PDScore,
                                                                                        error: *const c_char) {
    let res = if score.is_null() {
        Err(Error::from_ptr(error).expect("unable read err"))
    } else {
        if !error.is_null() {
            let err = Error::from_ptr(error).expect("unable read err");
            sys::println!("Err: {err}");
        }

        Ok(ScoreRef(score))
    };

    let f = STORE.lock().as_mut().map(|store| store.remove::<F>()).flatten();
    f.map(|mut f| f(res)).or_else(|| panic!("missed callback"));

    clean_store();
}
