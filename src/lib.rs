pub mod beat;
pub mod chart;
pub mod effect;
pub mod meta;
pub mod note;
pub mod song;
pub mod time;

pub use beat::Beat;
pub use chart::Chart;
pub use effect::Effect;
pub use meta::MetaData;
pub use note::Note;
pub use song::SongInfo;
pub use time::BpmEvent;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_pure_ruby() {
        let json = std::fs::read_to_string(
            "test_resource/Pure Ruby/0/1764168416.mc"
        )
        .unwrap();
        let chart = Chart::from(json);

        assert_eq!(chart.meta.ver, 0);
        assert_eq!(chart.meta.mode, 0);
        assert_eq!(chart.meta.song.title, "Pure Ruby");
        assert!(!chart.note.is_empty());
        assert!(!chart.time.is_empty());
        // Pure Ruby 有 1 个 sound 事件
        let sound_count = chart
            .note
            .iter()
            .filter(|n| matches!(n, Note::Sound { .. }))
            .count();
        assert_eq!(sound_count, 1);
    }

    #[test]
    fn parse_dan_chart() {
        let json = std::fs::read_to_string(
            "test_resource/_song_5000/0/00th Dan v3 (M.D.C.E Team).mc"
        )
        .unwrap();
        let chart = Chart::from(json);

        assert_eq!(chart.meta.song.title, "Malody 4K Dan v3 (Regular)");
        // 5 个 BPM 变速点
        assert_eq!(chart.time.len(), 5);
        // 6 个 scroll 变速效果
        assert_eq!(chart.effect.len(), 6);
        // 2298 个音符
        assert_eq!(chart.note.len(), 2298);

        let sound_count = chart
            .note
            .iter()
            .filter(|n| matches!(n, Note::Sound { .. }))
            .count();
        let normal_count = chart
            .note
            .iter()
            .filter(|n| matches!(n, Note::Normal { .. }))
            .count();
        assert_eq!(sound_count, 1);
        assert_eq!(normal_count, 2297);

        let hold_count = chart
            .note
            .iter()
            .filter(|n| matches!(n, Note::Normal { end_beat: Some(_), .. }))
            .count();
        assert_eq!(hold_count, 319);
    }

    #[test]
    fn parse_never_escape() {
        let json = std::fs::read_to_string(
            "test_resource/Never Escape/0/key_4k_hard.mc"
        )
        .unwrap();
        let chart = Chart::from(json);

        assert_eq!(chart.meta.song.title, "Never Escape");
        assert_eq!(chart.meta.creator, "ASDWADSXC");

        let sound_count = chart
            .note
            .iter()
            .filter(|n| matches!(n, Note::Sound { .. }))
            .count();
        let normal_count = chart
            .note
            .iter()
            .filter(|n| matches!(n, Note::Normal { .. }))
            .count();
        assert_eq!(sound_count, 3238);
        assert_eq!(normal_count, 10);

        // 有长条 (包含 keysounded hold 和普通 hold)
        let hold_count = chart
            .note
            .iter()
            .filter(|n| {
                matches!(
                    n,
                    Note::Sound { end_beat: Some(_), .. }
                        | Note::Normal { end_beat: Some(_), .. }
                )
            })
            .count();
        assert_eq!(hold_count, 24);

        // 带 column 的 keysound (出现在轨道上的按键音)
        let key_count = chart
            .note
            .iter()
            .filter(|n| matches!(n, Note::Sound { column: Some(_), .. }))
            .count();
        assert_eq!(key_count, 1190);
    }

    #[test]
    fn beat_roundtrip() {
        let json = r#"[[4,0,4],[4,383,768],[0,0,1],[261,595,768]]"#;
        let beats: Vec<Beat> = serde_json::from_str(json).unwrap();
        let output = serde_json::to_string(&beats).unwrap();
        let beats2: Vec<Beat> = serde_json::from_str(&output).unwrap();

        for (a, b) in beats.iter().zip(beats2.iter()) {
            assert_eq!(a, b);
        }
    }

    #[test]
    fn beat_serde_format() {
        let beat: Beat = serde_json::from_str("[4,383,768]").unwrap();
        assert_eq!(beat.integer(), 4);
        assert_eq!(beat.numerator(), 383);
        assert_eq!(beat.denominator(), 768);

        let output = serde_json::to_string(&beat).unwrap();
        assert_eq!(output, "[4,383,768]");
    }

    #[test]
    fn parse_never_escape_individual() {
        // 逐个反序列化每个 note，验证无分类错误
        let json = std::fs::read_to_string(
            "test_resource/Never Escape/0/key_4k_hard.mc"
        )
        .unwrap();
        let raw: serde_json::Value = serde_json::from_str(&json).unwrap();
        let notes = raw["note"].as_array().unwrap();

        let mut sound_ok = 0u32;
        let mut normal_ok = 0u32;

        for note_value in notes {
            let has_sound = note_value["sound"].is_string();
            let note: Note = serde_json::from_value(note_value.clone()).unwrap();
            match (&note, has_sound) {
                (Note::Sound { .. }, true) => sound_ok += 1,
                (Note::Normal { .. }, false) => normal_ok += 1,
                _ => panic!("misclassified note: {note_value}"),
            }
        }

        assert_eq!(sound_ok, 3238);
        assert_eq!(normal_ok, 10);
    }

    #[test]
    fn beat_normalization() {
        let a = Beat::new(0, 2, 4);
        assert_eq!(a.integer(), 0);
        assert_eq!(a.numerator(), 1);
        assert_eq!(a.denominator(), 2);

        // 假分数 carry 到整数部分
        let b = Beat::new(1, 5, 3);
        assert_eq!(b.integer(), 2);
        assert_eq!(b.numerator(), 2);
        assert_eq!(b.denominator(), 3);
    }
}
