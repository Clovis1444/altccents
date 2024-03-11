// tests.rs

// Unit tests for data.rs
#[test]
fn data_get_accent_test() {
    use super::data::*;

    {
        let sub_test = "\"key\" argument test";

        let result = get_accent(AccentKey::A, false, 0);
        assert_eq!(result, 'à', "{}", sub_test);

        let result = get_accent(AccentKey::O, false, 0);
        assert_eq!(result, 'ô', "{}", sub_test);

        let result = get_accent(AccentKey::U, false, 0);
        assert_eq!(result, 'ù', "{}", sub_test);
    }

    {
        let sub_test = "\"is_capital\" argument test";

        let result = get_accent(AccentKey::I, false, 0);
        assert_eq!(result, 'î', "{}", sub_test);

        let result = get_accent(AccentKey::I, true, 0);
        assert_eq!(result, 'Î', "{}", sub_test);

        let result = get_accent(AccentKey::E, false, 0);
        assert_eq!(result, 'é', "{}", sub_test);

        let result = get_accent(AccentKey::E, true, 0);
        assert_eq!(result, 'É', "{}", sub_test);
    }

    {
        let sub_test = "\"index\" argument test";

        let result = get_accent(AccentKey::A, false, 2);
        assert_eq!(result, 'â', "{}", sub_test);

        let result = get_accent(AccentKey::U, true, 1);
        assert_eq!(result, 'Û', "{}", sub_test);

        let result = get_accent(AccentKey::E, false, 3);
        assert_eq!(result, 'ë', "{}", sub_test);
    }

    {
        let sub_test = "forbidden \"AccentKey\" variant test";

        let result = get_accent(AccentKey::EnumLength, false, 0);
        assert_eq!(result, '?', "{}", sub_test);

        let result = get_accent(AccentKey::EnumLength, true, 0);
        assert_eq!(result, '?', "{}", sub_test);

        let result = get_accent(AccentKey::EnumLength, false, 100);
        assert_eq!(result, '?', "{}", sub_test);

        let result = get_accent(AccentKey::EnumLength, true, 100);
        assert_eq!(result, '?', "{}", sub_test);
    }

    {
        let sub_test = "\"is_capital\" argument test when no upper_case symbols in the AccentKey";

        let result = get_accent(AccentKey::Euro, false, 0);
        assert_eq!(result, '€', "{}", sub_test);

        let result = get_accent(AccentKey::Euro, true, 0);
        assert_eq!(result, '€', "{}", sub_test);
    }

    {
        let sub_test = "Wrong \"index\" parameter test";

        let result = get_accent(AccentKey::C, false, 1);
        assert_eq!(result, '?', "{}", sub_test);

        let result = get_accent(AccentKey::I, false, 2);
        assert_eq!(result, '?', "{}", sub_test);

        let result = get_accent(AccentKey::A, false, 200);
        assert_eq!(result, '?', "{}", sub_test);
    }
}

#[test]
fn data_accent_key_vk_test() {
    use super::data::*;
    use windows::Win32::UI::Input::KeyboardAndMouse::*;

    let sub_test = "\"AccentKey\" variant binding test";

    let result = AccentKey::A.vk();
    assert_eq!(result, Some(VK_A), "{}", sub_test);

    let result = AccentKey::E.vk();
    assert_eq!(result, Some(VK_E), "{}", sub_test);

    let result = AccentKey::I.vk();
    assert_eq!(result, Some(VK_I), "{}", sub_test);

    let result = AccentKey::O.vk();
    assert_eq!(result, Some(VK_O), "{}", sub_test);

    let result = AccentKey::U.vk();
    assert_eq!(result, Some(VK_U), "{}", sub_test);

    let result = AccentKey::C.vk();
    assert_eq!(result, Some(VK_C), "{}", sub_test);

    let result = AccentKey::Y.vk();
    assert_eq!(result, Some(VK_Y), "{}", sub_test);

    let result = AccentKey::Euro.vk();
    assert_eq!(result, Some(VK_OEM_7), "{}", sub_test);

    let result = AccentKey::EnumLength.vk();
    assert_eq!(result, None, "{}", sub_test);
}

#[test]
fn data_accent_key_from_msg_test() {
    use super::data::*;
    use windows::Win32::Foundation::WPARAM;
    use windows::Win32::UI::{
        Input::KeyboardAndMouse::*,
        WindowsAndMessaging::{MSG, WM_KEYDOWN},
    };

    let sub_test = "\"Virtual key\" binding test";

    let mut msg = MSG::default();
    msg.message = WM_KEYDOWN;

    msg.wParam = WPARAM { 0: VK_A.0 as usize };
    let result = AccentKey::from_msg(&msg);
    assert_eq!(result, Some(AccentKey::A), "{}", sub_test);

    msg.wParam = WPARAM { 0: VK_E.0 as usize };
    let result = AccentKey::from_msg(&msg);
    assert_eq!(result, Some(AccentKey::E), "{}", sub_test);

    msg.wParam = WPARAM { 0: VK_I.0 as usize };
    let result = AccentKey::from_msg(&msg);
    assert_eq!(result, Some(AccentKey::I), "{}", sub_test);

    msg.wParam = WPARAM { 0: VK_O.0 as usize };
    let result = AccentKey::from_msg(&msg);
    assert_eq!(result, Some(AccentKey::O), "{}", sub_test);

    msg.wParam = WPARAM { 0: VK_U.0 as usize };
    let result = AccentKey::from_msg(&msg);
    assert_eq!(result, Some(AccentKey::U), "{}", sub_test);

    msg.wParam = WPARAM { 0: VK_C.0 as usize };
    let result = AccentKey::from_msg(&msg);
    assert_eq!(result, Some(AccentKey::C), "{}", sub_test);

    msg.wParam = WPARAM { 0: VK_Y.0 as usize };
    let result = AccentKey::from_msg(&msg);
    assert_eq!(result, Some(AccentKey::Y), "{}", sub_test);

    msg.wParam = WPARAM {
        0: VK_OEM_7.0 as usize,
    };
    let result = AccentKey::from_msg(&msg);
    assert_eq!(result, Some(AccentKey::Euro), "{}", sub_test);

    msg.wParam = WPARAM { 0: VK_S.0 as usize };
    let result = AccentKey::from_msg(&msg);
    assert_eq!(result, None, "{}", sub_test);
}
