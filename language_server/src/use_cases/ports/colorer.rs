use crate::entities::{Color, ColorType, Colors, Column, Match, RowMetadata};

pub trait Colorer {
    fn color_text(&self, text: &str, color_type: ColorType) -> Option<Color>;

    fn update_palette(&mut self, text: String, colors: Colors);

    fn color_intervals(
        &self,
        row_matches: &[Match],
        row_meta: &RowMetadata,
    ) -> Vec<(Column, Column)> {
        let mut color_intervals = vec![];

        color_intervals.extend(
            row_matches
                .iter()
                .zip(row_matches.iter().skip(1))
                .map(|(match1, match2)| (*match1.column(), *match2.column())),
        );

        let last_match = row_matches
                .last()
                .expect("This is always called when there is a match in user's text so there is something in the slice");
        let line_end = (row_meta.line_len() - 1).into();
        color_intervals.push((*last_match.column(), line_end));

        // Extend the first match interval to include the text up until the
        // first non-zero column
        color_intervals[0].0 = *row_meta.first_non_zero_pos();
        color_intervals
    }
}

// grcov-excl-start
#[cfg(test)]
mod tests {
    use mockall::mock;

    use super::*;

    mock! {
           Highlighter {}

           impl Colorer for Highlighter {
               fn color_text(&self, text: &str, color_type: ColorType) -> Option<Color>;

               fn update_palette(&mut self, text: String, colors: Colors);
           }
    }

    #[test]
    fn one_match_one_interval() {
        let row_matches = vec![Match::new(3.into(), "keyword1".to_owned())];
        let line_len = 20;
        let row_meta = RowMetadata::new(line_len, 0.into());

        assert_eq!(
            MockHighlighter::new().color_intervals(&row_matches, &row_meta),
            vec![(0.into(), (line_len - 1).into())]
        );
    }

    #[test]
    fn color_intervals_are_crrectly_deteted() {
        let row_matches = vec![
            Match::new(3.into(), "keyword1".to_owned()),
            Match::new(18.into(), "another-key".to_owned()),
            Match::new(34.into(), "aword".to_owned()),
        ];
        let line_len = 80;
        let row_meta = RowMetadata::new(line_len, 0.into());

        assert_eq!(
            MockHighlighter::new().color_intervals(&row_matches, &row_meta),
            vec![
                (0.into(), 18.into()),
                (18.into(), 34.into()),
                (34.into(), (line_len - 1).into())
            ]
        );
    }
}
// grcov-excl-start
