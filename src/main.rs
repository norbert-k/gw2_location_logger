use clap::{AppSettings, Clap};
use mumblelink_reader::mumble_link_handler::MumbleLinkHandler;
use mumblelink_reader::mumble_link::{MumbleLinkReader, MumbleLinkDataReader, MumbleLinkData};
use std::{thread, time};
use serde::Serialize;
use std::path::Path;
use csv::Writer;
use std::io::Write;

#[derive(Clap)]
#[clap(version = "1.0", author = "Norberts K. <https://github.com/norbert-k>")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Options {
    #[clap(short, long, default_value = "1000")]
    update_rate: u32,
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap, Debug)]
enum SubCommand {
    #[clap(version = "1.0", author = "Norberts K. <https://github.com/norbert-k>")]
    FileOutput(FileOutputOptions),
    #[clap(version = "1.0", author = "Norberts K. <https://github.com/norbert-k>")]
    ConsoleOutput,
}

#[derive(Clap, Debug)]
struct FileOutputOptions {
    #[clap(short, long, default_value = "./gw2_location_log.csv")]
    output_file_path: String,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
struct GuildwarsContext {
    pub server_address: [u8; 28],
    pub map_id: u32,
    pub map_type: u32,
    pub shard_id: u32,
    pub instance: u32,
    pub build_id: u32,
    pub ui_state: u32,
    pub compass_width: u16,
    pub compass_height: u16,
    pub compass_rotation: f32,
    pub player_x: f32,
    pub player_y: f32,
    pub map_center_x: f32,
    pub map_center_y: f32,
    pub map_scale: f32,
    pub process_id: u32,
    pub mount_index: u8,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
struct MumbleLinkCsv {
    camera_position_x: f32,
    camera_position_y: f32,
    camera_position_z: f32,
    camera_front_x: f32,
    camera_front_y: f32,
    camera_front_z: f32,
    camera_top_x: f32,
    camera_top_y: f32,
    camera_top_z: f32,
    player_x: f32,
    player_y: f32,
    map_center_x: f32,
    map_center_y: f32,
    compass_rotation: f32,
    map_id: u32,
}

impl MumbleLinkCsv {
    fn from_mumblelink_data(mumblelink_data: &MumbleLinkData) -> MumbleLinkCsv {
        let guildwars_context = mumblelink_data.read_context_into_struct::<GuildwarsContext>();
        MumbleLinkCsv {
            camera_position_x: mumblelink_data.camera.position[0],
            camera_position_y: mumblelink_data.camera.position[1],
            camera_position_z: mumblelink_data.camera.position[2],
            camera_front_x: mumblelink_data.camera.front[0],
            camera_front_y: mumblelink_data.camera.front[0],
            camera_front_z: mumblelink_data.camera.front[0],
            camera_top_x: mumblelink_data.camera.top[0],
            camera_top_y: mumblelink_data.camera.top[0],
            camera_top_z: mumblelink_data.camera.top[0],
            player_x: guildwars_context.player_x,
            player_y: guildwars_context.player_y,
            map_center_x: guildwars_context.map_center_x,
            map_center_y: guildwars_context.map_center_y,
            compass_rotation: guildwars_context.compass_rotation,
            map_id: guildwars_context.map_id,
        }
    }
}

fn write_csv<T: Write>(handler: &MumbleLinkHandler, writer: &mut Writer<T>, update_rate: u32) {
    loop {
        let linked_memory = handler.read().unwrap();
        let csv_data = MumbleLinkCsv::from_mumblelink_data(&linked_memory);
        writer.serialize(csv_data);
        writer.flush();
        thread::sleep(time::Duration::from_millis(update_rate as u64));
    }
}

fn file_output_cmd(handler: &MumbleLinkHandler, file_options: &FileOutputOptions, update_rate: u32) {
    println!("Output file path: {}", file_options.output_file_path);
    let mut writer = csv::Writer::from_path(file_options.output_file_path.clone()).unwrap();
    write_csv(&handler, &mut writer, update_rate);
}

fn console_output_cmd(handler: &MumbleLinkHandler, update_rate: u32) {
    let mut writer = csv::Writer::from_writer(std::io::stdout());
    write_csv(&handler, &mut writer, update_rate);
}

fn main() {
    let options: Options = Options::parse();
    let handler = MumbleLinkHandler::new().unwrap();
    match options.subcmd {
        SubCommand::FileOutput(file_options) => file_output_cmd(&handler, &file_options, options.update_rate),
        SubCommand::ConsoleOutput => console_output_cmd(&handler, options.update_rate)
    }
}
