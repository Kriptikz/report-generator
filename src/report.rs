use docx_rs::*;
use crate::test::*;
use crate::client::*;

pub fn generate_report(client: &Client, test: &Test) -> Result<(), DocxError> {
    
    let mut full_file_path: String = String::new();
    full_file_path.push_str(&String::from("./reports/report.docx"));
    let path = std::path::Path::new(&full_file_path);
    let file = std::fs::File::create(&path).unwrap();

    let mut client_name = String::new();
    client_name.push_str("Client Name: ");
    client_name.push_str(client.get_name());

    let mut client_age = String::new();
    client_age.push_str("        Age: ");
    client_age.push_str(&(client.get_age()).to_string());

    let mut client_description = String::new();
    client_description.push_str(&client_name);
    client_description.push_str(&client_age);

    // convert to string slice
    let client_description = &client_description[..];

    // ----------------------------------------------------
    // Verbal Comprehension (VCI)
    let waisiv_si_scaled_score = &*get_score_as_string(client, test, &String::from("Verbal Comprehension"), &String::from("Similarities"));
    let waisiv_vc_scaled_score = &*get_score_as_string(client, test, &String::from("Verbal Comprehension"), &String::from("Vocabulary"));
    let waisiv_in_scaled_score = &*get_score_as_string(client, test, &String::from("Verbal Comprehension"), &String::from("Information"));
    let waisiv_co_scaled_score = &*get_score_as_string(client, test, &String::from("Verbal Comprehension"), &String::from("Comprehension"));

    let vci_sum: u32 = get_u32_from_string(&String::from(waisiv_si_scaled_score))
        + get_u32_from_string(&String::from(waisiv_vc_scaled_score))
        + get_u32_from_string(&String::from(waisiv_in_scaled_score));

    let waisiv_vci_sum_score = get_scaled_score_equivalent_as_string(client, test, &String::from("Verbal Comprehension"), &String::from("Similarities"), vci_sum);


    // ----------------------------------------------------
    // Perceptual Reasoning (PRI)
    let waisiv_bd_scaled_score = &*get_score_as_string(client, test, &String::from("Perceptual Reasoning"), &String::from("Block Design"));
    let waisiv_mr_scaled_score = &*get_score_as_string(client, test, &String::from("Perceptual Reasoning"), &String::from("Matrix Reasoning"));
    let waisiv_vp_scaled_score = &*get_score_as_string(client, test, &String::from("Perceptual Reasoning"), &String::from("Visual Puzzles"));
    let waisiv_fw_scaled_score = &*get_score_as_string(client, test, &String::from("Perceptual Reasoning"), &String::from("Figure Weights"));
    let waisiv_pc_scaled_score = &*get_score_as_string(client, test, &String::from("Perceptual Reasoning"), &String::from("Picture Completion"));

    let pri_sum: u32 = get_u32_from_string(&String::from(waisiv_bd_scaled_score))
    + get_u32_from_string(&String::from(waisiv_mr_scaled_score))
    + get_u32_from_string(&String::from(waisiv_vp_scaled_score))
    + get_u32_from_string(&String::from(waisiv_fw_scaled_score))
    + get_u32_from_string(&String::from(waisiv_pc_scaled_score));

    let waisiv_pri_sum_score = get_scaled_score_equivalent_as_string(client, test, &String::from("Perceptual Reasoning"), &String::from("Similarities"), pri_sum);

    // ----------------------------------------------------
    // Working Memory (WMI)
    let waisiv_ds_scaled_score = &*get_score_as_string(client, test, &String::from("Working Memory"), &String::from("Digit Span"));
    let waisiv_ar_scaled_score = &*get_score_as_string(client, test, &String::from("Working Memory"), &String::from("Arithmetic"));
    let waisiv_ln_scaled_score = &*get_score_as_string(client, test, &String::from("Working Memory"), &String::from("Letter-Number Seq."));

    let wmi_sum: u32 = get_u32_from_string(&String::from(waisiv_ds_scaled_score))
    + get_u32_from_string(&String::from(waisiv_ar_scaled_score))
    + get_u32_from_string(&String::from(waisiv_ln_scaled_score));

    let waisiv_wmi_sum_score = get_scaled_score_equivalent_as_string(client, test, &String::from("Working Memory"), &String::from("Similarities"), wmi_sum);


    // ----------------------------------------------------
    // Processing Speed (PSI)
    let waisiv_ss_scaled_score = &*get_score_as_string(client, test, &String::from("Processing Speed"), &String::from("Symbol Search"));
    let waisiv_cd_scaled_score = &*get_score_as_string(client, test, &String::from("Processing Speed"), &String::from("Coding"));
    let waisiv_ca_scaled_score = &*get_score_as_string(client, test, &String::from("Processing Speed"), &String::from("Cancellation"));

    let psi_sum: u32 = get_u32_from_string(&String::from(waisiv_ss_scaled_score))
    + get_u32_from_string(&String::from(waisiv_cd_scaled_score))
    + get_u32_from_string(&String::from(waisiv_ca_scaled_score));

    let waisiv_psi_sum_score = get_scaled_score_equivalent_as_string(client, test, &String::from("Processing Speed"), &String::from("Similarities"), psi_sum);

    let test_description = "The WAIS-IV is";
    Docx::new()
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("WAIS-IV:").bold()))
        .add_paragraph(make_paragraph(client_description, AlignmentType::Left, "none"))
        .add_paragraph(make_paragraph(test_description, AlignmentType::Left, "none"))
        .add_paragraph(Paragraph::new())
        .add_table(
            Table::new(vec![
                TableRow::new(vec![
                    make_cell("Index", AlignmentType::Left, "single"),
                    make_cell("Standard Score", AlignmentType::Center, "single"),
                    make_cell("Percentile", AlignmentType::Center, "single"),
                    make_cell("Qualitative Description", AlignmentType::Center, "single"),
                ]),
                TableRow::new(vec![
                    make_cell("Verbal Comprehension (VCI)", AlignmentType::Left, "none"),
                    make_cell(&waisiv_vci_sum_score.0, AlignmentType::Center, "none"),
                    make_cell(&waisiv_vci_sum_score.1, AlignmentType::Center, "none"),
                    make_cell("WIP", AlignmentType::Center, "none"),
                ]),
                TableRow::new(vec![
                    make_cell("Perceptual Reasoning (PRI)", AlignmentType::Left, "none"),
                    make_cell(&waisiv_pri_sum_score.0, AlignmentType::Center, "none"),
                    make_cell(&waisiv_pri_sum_score.1, AlignmentType::Center, "none"),
                    make_cell("WIP", AlignmentType::Center, "none"),
                ]),
                TableRow::new(vec![
                    make_cell("Working Memory (WMI)", AlignmentType::Left, "none"),
                    make_cell(&waisiv_wmi_sum_score.0, AlignmentType::Center, "none"),
                    make_cell(&waisiv_wmi_sum_score.1, AlignmentType::Center, "none"),
                    make_cell("WIP", AlignmentType::Center, "none"),
                ]),
                TableRow::new(vec![
                    make_cell("Processing Speed (PSI)", AlignmentType::Left, "none"),
                    make_cell(&waisiv_psi_sum_score.0, AlignmentType::Center, "none"),
                    make_cell(&waisiv_psi_sum_score.1, AlignmentType::Center, "none"),
                    make_cell("WIP", AlignmentType::Center, "none"),
                ]),
                TableRow::new(vec![
                    make_cell("Full Scale IQ (FSIQ)", AlignmentType::Left, "none"),
                    make_cell(&waisiv_psi_sum_score.0, AlignmentType::Center, "none"),
                    make_cell(&waisiv_psi_sum_score.1, AlignmentType::Center, "none"),
                    make_cell("WIP", AlignmentType::Center, "none"),
                ]),
            ]) 
            .set_grid(vec![2000, 6500, 2000])
            .layout(TableLayoutType::Fixed)
            .indent(0))
        .add_paragraph(Paragraph::new())
        .add_table(
            Table::new(vec![
                TableRow::new(vec![
                    make_cell("Verbal Comprehension", AlignmentType::Left, "single"),
                    make_cell("Standard Score", AlignmentType::Center, "single"),
                    make_cell("Perceptual Reasoning", AlignmentType::Left, "single"),
                    make_cell("Standard Score", AlignmentType::Center, "single"),
                ]),
                TableRow::new(vec![
                    make_cell("Similarities (SI)", AlignmentType::Left, "none"),
                    make_cell(waisiv_si_scaled_score, AlignmentType::Center, "none"),
                    make_cell("Block Design (BD)", AlignmentType::Left, "none"),
                    make_cell(waisiv_bd_scaled_score, AlignmentType::Center, "none"),
                ]),
                TableRow::new(vec![
                    make_cell("Vocabulary (VC)", AlignmentType::Left, "none"),
                    make_cell(waisiv_vc_scaled_score, AlignmentType::Center, "none"),
                    make_cell("Matrix Reasoning (MR)", AlignmentType::Left, "none"),
                    make_cell(waisiv_mr_scaled_score, AlignmentType::Center, "none"),
                ]),
                TableRow::new(vec![
                    make_cell("Information (IN)", AlignmentType::Left, "none"),
                    make_cell(waisiv_in_scaled_score, AlignmentType::Center, "none"),
                    make_cell("Visual Puzzles (VP)", AlignmentType::Left, "none"),
                    make_cell(waisiv_vp_scaled_score, AlignmentType::Center, "none"),
                ]),
                TableRow::new(vec![
                    make_cell("Comprehension (CO)", AlignmentType::Left, "none"),
                    make_cell(waisiv_co_scaled_score, AlignmentType::Center, "none"),
                    make_cell("Figure Weights (FW)", AlignmentType::Left, "none"),
                    make_cell(waisiv_fw_scaled_score, AlignmentType::Center, "none"),
                ]),
                TableRow::new(vec![
                    make_cell("", AlignmentType::Left, "none"),
                    make_cell("", AlignmentType::Left, "none"),
                    make_cell("Picture Completion (PC)", AlignmentType::Left, "none"),
                    make_cell(waisiv_pc_scaled_score, AlignmentType::Center, "none"),
                ]),
                TableRow::new(vec![
                    make_cell("Working Memory", AlignmentType::Left, "single"),
                    make_cell("Standard Score", AlignmentType::Center, "single"),
                    make_cell("Processing Speed", AlignmentType::Left, "single"),
                    make_cell("Standard Score", AlignmentType::Center, "single"),
                ]),
                TableRow::new(vec![
                    make_cell("Digit Span (DS)", AlignmentType::Left, "none"),
                    make_cell(waisiv_ds_scaled_score, AlignmentType::Center, "none"),
                    make_cell("Symbol Search (SS)", AlignmentType::Left, "none"),
                    make_cell(waisiv_ss_scaled_score, AlignmentType::Center, "none"),
                ]),
                TableRow::new(vec![
                    make_cell("Arithmetic (AR)", AlignmentType::Left, "none"),
                    make_cell(waisiv_ar_scaled_score, AlignmentType::Center, "none"),
                    make_cell("Coding (CD)", AlignmentType::Left, "none"),
                    make_cell(waisiv_cd_scaled_score, AlignmentType::Center, "none"),
                ]),
                TableRow::new(vec![
                    make_cell("Letter-Number Seq. (LN)", AlignmentType::Left, "none"),
                    make_cell(waisiv_ln_scaled_score, AlignmentType::Center, "none"),
                    make_cell("Cancelation (CA)", AlignmentType::Left, "none"),
                    make_cell(waisiv_ca_scaled_score, AlignmentType::Center, "none"),
                ]),
            ]) 
            .set_grid(vec![2000, 6500, 2000])
            .layout(TableLayoutType::Fixed)
            .indent(0))
        .page_margin(PageMargin {top: 10, left: 500, bottom: 10, right: 500, header: 10, footer: 10, gutter: 10})
        .build()
        .pack(file)?;

    Ok(())
}

fn make_paragraph(text: &str, text_alignment: AlignmentType, underline_line_type: &str) -> Paragraph {
    Paragraph::new().add_run(Run::new().add_text(text).underline(underline_line_type)).align(text_alignment)
}

fn make_cell(text: &str, text_alignment: AlignmentType, underline_line_type: &str) -> TableCell {
    TableCell::new().add_paragraph(make_paragraph(text, text_alignment, underline_line_type))
}

fn get_u32_from_string(string: &String) -> u32 {
    match string.parse::<u32>() {
        Ok(number) => return number,
        Err(_) => (),
    }

    0
}

fn get_score_as_string(client: &Client, test: &Test, index_name: &String, subtest_name: &String) -> String {

    let mut string_scaled_score = String::new();
    match client.get_test_score(test.get_name().to_string(), index_name.to_string(), subtest_name.to_string())
    {
        Some(score) => {
            match get_scaled_score(client, test, index_name, subtest_name, score) {
                Some(scaled_score) => string_scaled_score.push_str(&scaled_score.to_string()),
                None => string_scaled_score.push('x'),
            }
        },
        None => string_scaled_score.push('x'),
    }


    string_scaled_score
}

fn get_scaled_score(client: &Client, test: &Test, index_name: &String, subtest_name: &String, score: u32) -> Option<u32> {
    match test.get_index(index_name.to_string()) {
        Some(index) => {
            match index.get_subtest(subtest_name.to_string()) {
                Some(subtest) => {
                    match subtest.get_chart(client.get_age()) {
                        Some(chart) => {
                            let mut scaled_score = chart.get_scaled_score_range().get_min();
                            for raw_score_max in &mut chart.get_raw_score_maxes().iter() {
                                if score <= *raw_score_max {
                                    return Some(scaled_score)
                                }
                                scaled_score += 1;
                            }
                            
                        }
                        None => (),
                    }
                }
                None => (),
            }
        }
        None => (),
    }

    None
}

fn get_scaled_score_equivalent_as_string(client: &Client, test: &Test, index_name: &String, subtest_name: &String, score: u32) -> (String, String) {

    let mut string_scaled_score = String::new();
    let mut string_rank = String::new();

    match get_scaled_score_equivalent(client, test, index_name, subtest_name, score) {
        Some(scaled_score) => {
            string_scaled_score.push_str(&scaled_score.0.to_string());
            string_rank.push_str(&scaled_score.1.to_string());
        },
        None => string_scaled_score.push('x'),
    }


    (string_scaled_score, string_rank)
}

fn get_scaled_score_equivalent(client: &Client, test: &Test, index_name: &String, subtest_name: &String, score: u32) -> Option<(u32, f32)> {
    match test.get_index(index_name.to_string()) {
        Some(index) => {
            // find out spot in the raw_score_maxes_equivalents using a counter
            let mut counter: u32 = 0;
            for sum in index.get_equivalents_chart().get_scaled_score_range().get_min()..index.get_equivalents_chart().get_scaled_score_range().get_max() {
                if sum == score {
                    break;
                }
                counter += 1;
            }

            let equivalent = index.get_equivalents_chart().get_raw_score_maxes()[counter as usize];
            let rank = index.get_equivalents_chart().get_percentile_ranks()[counter as usize];

            return Some((equivalent, rank))
        }
        None => (),
    }

    None 
}

