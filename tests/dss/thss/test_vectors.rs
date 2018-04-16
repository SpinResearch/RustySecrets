use rusty_secrets::dss::thss::{recover_secret, MetaData, Share};

const THRESHOLD: u8 = 5;
const SHARES_COUNT: u8 = 7;
const SECRET: &'static [u8] =
    b"These programs were never about terrorism: they're about economic spying, \
      social control, and diplomatic manipulation. They're about power.";

fn meta_data() -> MetaData {
    let mut metadata = MetaData::new();
    metadata
        .tags
        .insert("mime_type".to_string(), "text/plain".to_string());
    metadata
}

#[cfg_attr(rustfmt, rustfmt_skip)]
fn shares_meta() -> Vec<&'static str> {
    vec!["5-1-CAEQBRgHIosByvXQoy+4Cg/UFieZRaQXJU+PW508oor5AQX+7aU+i+qAn3ope5AH/HvVsoQNaoi6zKJOEgoru0etz0BLhgqyyTuw+K0MKtdiS3dw5JHiUJBfDf1lwZIserX2cFhLLgRYRA0stuFkAmNeQvy9Mhn0m/gEfPSI9HxIS78Aqeo/RATgkgO6/Zwxl3l98zIZChcKCW1pbWVfdHlwZRIKdGV4dC9wbGFpbg",
    "5-2-CAIQBRgHIosBm/oF5oOAmoA+8A9q2fWXhQTcq9WUxlwGSaeEVQSvrtNL/UjcEioBgwQ2vqsI5sBAY6A0PSK1AS1PuPpY+A1WDtyE0dwGoTKkh7qfyxZ+iIfFcDtHS8fBFr58FWGePGyoFDGgCByubmxgDsTR8PQpGjdN0B6Kq2LMwaZ1n6Hj8wnfKMHBbRthxK/mkzIZChcKCW1pbWVfdHlwZRIKdGV4dC9wbGFpbg",
    "5-3-CAMQBRgHIosBNcVtExWApxy1LqVUoFJ+lG0gIayk7u0G16aApzZwcg2AsYWIhz0p4d0mb+itpZP8k6rk6uMVLiNUu0QZK87QgH6F20JXx69qqohEp/Ee9ZsTe74FlCDWBAwFi73jXnfnYKZBfDl1DglGQStuGi8GbC08Dq6VrAYTp89UnqbBKyuzOphOTEIE0jJJ4DIZChcKCW1pbWVfdHlwZRIKdGV4dC9wbGFpbg",
    "5-4-CAQQBRgHIosBFivrJmaYVpCb6NEYR/glr/mIQLNilCB3M71myh4SsKah1hsZcjwniZdIiWOFj8ZIYBMEGfF3r96hegWY2rnzaN/has1c+TdSnmGJcX0eXz2lcbEooF/HNJUyObvuhjvMPreKl90awaVgEqy3fSu2iZh4/GowJz94UMKzyCwtVXtC1nIPkgWSnUHRhTIZChcKCW1pbWVfdHlwZRIKdGV4dC9wbGFpbg",
    "5-5-CAUQBRgHIosBKF2qKN53o5GAXLxOlBLsakRz4axhC2+EFECEVv4GhRTXv7dgR9rAUhmfjB5aNdwTZ0DJlUhRHWN/AU//qC/nLKEwwIfxZFbXWHVgkvHV27sLXFGIdK1hOvYhLcsw/Vzkl9giiaSKkObTYAARp8n0XywOaQv6CfPU8JUHQ9F6npGec+ZafXBWc3w5mzIZChcKCW1pbWVfdHlwZRIKdGV4dC9wbGFpbg",
    "5-6-CAYQBRgHIosBRMAthi6MvjlXbgdtQdksf7ouRyivHFiAMwcvMuYcb/V7l0fPc59FQsjvNU2rVgY6O/CJDJDegHIKhgCgiYI6YuO5tEMeBFaHX/TrvqAC7IVhbV1z6NLzbkJ/Qbe+3czaYAk84+HWnqXaVr5PBVbzg6JJU15PBKC26/BFfHNMDxHcuqOIbK9ZzfcsITIZChcKCW1pbWVfdHlwZRIKdGV4dC9wbGFpbg",
    "5-7-CAcQBRgHIosBShSxrUr7DNl8dZX9w0M7+UTUUvPObFrv+d5W0/6ceVMySC65x4zkdInHc4O0oL5Acm76JIAc0/JCVNeiwLJ3CGf5n0qXvBPJ04XaujkiJONmJaqaVicBZArD2i0yytCMiZA2Uj2NxIMxRGCFbgYf1JU+DRR891TZDhSpeBFjeLzu8Bicf2+m1UtkkTIZChcKCW1pbWVfdHlwZRIKdGV4dC9wbGFpbg"]
}

#[cfg_attr(rustfmt, rustfmt_skip)]
fn shares_no_meta() -> Vec<&'static str> {
    vec![
        "5-1-CAEQBRgIIosBnXo/a91D0XoED+H/h/jcYgcgEo7EvSIFmvFZmIqLUKC6uZ1u6Xnrub0KTWlmKWf+qGpmx4xbPq8Tu+TY1ng8GI1AP5U8+h0+EOHctgaQxRJi59u2HBBn0AGIlZTW/1e3USO4wQElC+HOX5TN9dWKD9+HZvKQjYGHwHGauAdMxOD73HNWeKRndO569Q",
        "5-2-CAIQBRgIIosBAaUa6GAfBauphwPYraTSQ6tlaqByOScscq4Xb83sI72+C+2JkdQxzowIUE7F1/RUAsmtealHAnoaegOlZdW42o2qEWD/2FgdRf0NpaW8NX1IgNZvZo9FhYCALJjSgLpacnXmTm6IMsNSYIN5YC7r8VBWwuUEzBj6WbbdtDqFYJ4kN46L+yu5hGbt8g",
        "5-3-CAMQBRgIIosBQELhGuZCPico0ploSo8bvCbrIoF2IVTsgwdlv2L4+0E+7y725QeUDJf7+yiklZOTa+Sh3glbE/XvQfC2w+en2p8UiIbAaGSAfobl/Xx9YfZZis1oAT29OMjAC89BJgJS6mIDTt895gosiOdAqswTs/ZRxPdqa7TQ74u6+ibPnDGcWyMWxuLi7sueZQ",
        "5-4-CAQQBRgIIosBzVzxXYLSdzKg4nrbCnl5BIiQCfj3FvcL7JP0tawF1FDv0AEVFISxIYPBSUg1KVEbN9vNov186DxmYlHd7B6SSVL6Bft5yJ06ZgvFDC7O9zltKIQ87x0PBE2Yrf17pADVh9imhTC4lwkItCCPBerspk5V3qQLyKWETp2EqfBIllCbM90FU3KyEjvO1g",
        "5-5-CAUQBRgIIosBE0pKA741rQWNBN+hzhU7V7Iw/FOedHGMUk9ts+i9wsWYnAZehGXh29NBilJEQUSGYBgqAqq4YgL72POVENd6A45kjRQ9kVHBSHHl9OSnSUPYr/UcpaUdviN6ic4PcC7FF1rd5oyLZiyhYJRXcMABA6Ck5zyycyp7on9Y6u42IjXEeAaCSOhNemTBuA",
        "5-6-CAYQBRgIIosBrGrvxWoApr9l90MPosc+M20p43TyBIMrJPro9WSfMApvY+PRKazM3HSlR43H6zOYtnoqsnwJdagiBrZeF5ENVQ/OgfMIfOkuN2+5umHGcNOnzyyLhS725e4rNgrY6/IYJDuir/k3FcuOJz48hrZKLr+EPSKVCvWxjxt0/NeXFsLI3Rdrh8HGjhWuUA",
        "5-7-CAcQBRgIIosB+on1cWjZ5gyiLO9bawtJirhVKVA1piIMg1472WrN2uNWF8j5S5W7Lu/mQuzZjV8bUpuHE2XlpcJ5WWCy9X2rbi+O3B82BmoRHq/ewxeXPlxBxvJ0wEYUXuUhwZSNRlfeDeHrYJTgUqW3KRt+pdugqkmBDTW8tXe3fdAsbqCKuom2SWdT+UZqj2zacg",
        "5-8-CAgQBRgIIosB4Izvun5lgaMwUNo6mFbAx1H2NnX//FxBBC0cuxJCeXsjw4BpT4IlTq5/u2hJg07MSG7D0Mi8hLsHsj129Gb1iUkOZQJH+Stsaodg2mJHXLyUwZBhHp1N5QZuNgHX7ayAqohfaZMqo0304wxkxpkwaNtFtnLsZkJuj2FMFNCFsDYz2W5gFur8QJWZRg"
    ]
}

#[test]
fn recover_n_shares_no_meta() {
    let shares = shares_no_meta()
        .iter()
        .cloned()
        .map(Share::from_string)
        .map(|s| s.unwrap())
        .collect::<Vec<_>>();

    let (recovered, metadata) = recover_secret(&shares).unwrap();

    assert_eq!(recovered, SECRET);
    assert_eq!(metadata, None);
}

#[test]
fn recover_k_shares_no_meta() {
    let shares = shares_no_meta()
        .iter()
        .cloned()
        .skip(1)
        .take(5)
        .map(Share::from_string)
        .map(|s| s.unwrap())
        .collect::<Vec<_>>();

    let (recovered, metadata) = recover_secret(&shares).unwrap();

    assert_eq!(recovered, SECRET);
    assert_eq!(metadata, None);
}

#[test]
#[should_panic(expected = "MissingShares")]
fn recover_less_shares_no_meta() {
    let shares = shares_no_meta()
        .iter()
        .cloned()
        .skip(1)
        .take(THRESHOLD as usize - 1)
        .map(Share::from_string)
        .map(|s| s.unwrap())
        .collect::<Vec<_>>();

    recover_secret(&shares).unwrap();
}

#[test]
fn recover_n_shares_meta() {
    let shares = shares_meta()
        .iter()
        .cloned()
        .map(Share::from_string)
        .map(|s| s.unwrap())
        .collect::<Vec<_>>();

    let (recovered, metadata) = recover_secret(&shares).unwrap();

    assert_eq!(recovered, SECRET);
    assert_eq!(metadata, Some(meta_data()));
}

#[test]
fn recover_k_shares_meta() {
    let shares = shares_meta()
        .iter()
        .cloned()
        .skip(1)
        .take(5)
        .map(Share::from_string)
        .map(|s| s.unwrap())
        .collect::<Vec<_>>();

    let (recovered, metadata) = recover_secret(&shares).unwrap();

    assert_eq!(recovered, SECRET);
    assert_eq!(metadata, Some(meta_data()));
}

#[test]
#[should_panic(expected = "MissingShares")]
fn recover_less_shares_meta() {
    let shares = shares_meta()
        .iter()
        .cloned()
        .skip(1)
        .take(THRESHOLD as usize - 1)
        .map(Share::from_string)
        .map(|s| s.unwrap())
        .collect::<Vec<_>>();

    recover_secret(&shares).unwrap();
}
