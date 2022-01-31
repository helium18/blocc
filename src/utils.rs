use crate::body;
use crate::document;
use crate::Config;
use crate::Coordinates;
use crate::Element;
use crate::JsValue;

pub fn update_grid<'a>(
    grid: &'a mut Vec<Element>,
    cords: &Coordinates,
    config: &Config,
) -> Result<&'a Vec<Element>, JsValue> {
    clear_grid(grid, config)?;

    let text = {
        let mut text = String::new();

        for x in 0..grid[cords.y as usize]
            .text_content()
            .ok_or_else(|| JsValue::from_str("Empty <p> tag"))?
            .trim()
            .chars()
            .count()
        {
            if x == cords.x as usize {
                text += format!("{}", config.filled_char).as_str();
            } else {
                text += format!("{}", config.empty_char).as_str();
            }
        }

        text
    };

    grid[cords.y as usize].set_text_content(Some(&text));

    Ok(grid)
}

pub fn clear_grid<'a>(
    grid: &'a mut Vec<Element>,
    config: &Config,
) -> Result<&'a mut Vec<Element>, JsValue> {
    for i in 0..config.rows {
        let mut text = String::new();
        for _ in 0..config.columns {
            text += format!("{}", config.empty_char).as_str();
        }

        grid[i as usize].set_text_content(Some(&text));
    }
    Ok(grid)
}

pub fn get_grid(config: &Config) -> Result<Vec<Element>, JsValue> {
    let mut grid: Vec<Element> = vec![];

    for i in 0..config.rows {
        let element = document().create_element("p")?;

        let mut text = String::new();
        for _ in 0..config.columns {
            text += format!("{}", config.empty_char).as_str();
        }

        element.set_text_content(Some(&text));

        body().append_child(&element)?;

        element.set_attribute("id", format!("grid-{}", i).as_str())?;
        grid.push(element);
    }

    Ok(grid)
}
