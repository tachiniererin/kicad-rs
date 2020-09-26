use lexpr::{self, parse::Options};
use std::fs;

#[derive(Debug, Default, Clone)]
struct Host {
    name: String,
    version: String,
}

#[derive(Debug, Default, Clone)]
struct General {
    thickness: f32,
    drawings: u32,
    tracks: u32,
    zones: u32,
    modules: u32,
    nets: u32,
}

type Point = (f32, f32);

#[derive(Debug, Default, Clone)]
struct Layer {
    num: u32,
    name: String,
    typ: String,
    hidden: bool,
}

#[derive(Debug, Default, Clone)]
struct PCBPlotParams {

}

#[derive(Debug, Default, Clone)]
struct Setup {
    
}

#[derive(Debug, Default, Clone)]
// Net declarations
struct Net {
    num: u32,
    name: String,
}

#[derive(Debug, Default, Clone)]
struct NetClass {
    name: String,
    label: String,
    clearance: f32,
    trace_width: f32,
    diff_pair_width: f32,
    diff_pair_gap: f32,
    via_dia: f32,
    via_drill: f32,
    uvia_dia: f32,
    uvia_drill: f32,
    nets: Vec<String>,
}

#[derive(Debug, Default, Clone)]
struct Effects {
    font_size: (f32, f32),
    thickness: f32,
}

#[derive(Debug, Default, Clone)]
struct FootprintText {
    typ: String,
    value: String,
    at: (f32, f32, f32),
    layer: String,
    hide: bool,
    effects: Effects,
}

#[derive(Debug, Default, Clone)]
struct FootprintLine {
    reference: String,
    start: (f32, f32),
    end: (f32, f32),
    layer: String,
    hide: bool,
    width: f32,
}

#[derive(Debug, Default, Clone)]
struct Pad {
    num: String, // can be int or string (e.g. for BGAs)
    pad_type: String, // smd, thr, tht
    typ: String, // roundrect, rect, circle
    at: (f32, f32, f32),
    size: (f32, f32),
    layers: Vec<String>,
    roundrect_rration: f32,
    net: Net,
    drill: f32,
    drill_oval: (f32, f32),
}

#[derive(Debug, Default, Clone)]
struct Model {
    path: String,
    at: (f32, f32, f32), // TODO: other coordinate systems than xyz???
    scale: (f32, f32, f32),
    rotate: (f32, f32, f32),
}

#[derive(Debug, Default, Clone)]
struct Module {
    name: String,
    layer: String,
    tedit: u32,
    tstamp: u32,
    at: (f32, f32, f32),
    descr: String,
    tags: String,
    path: String,
    attr: String,
    fp_text: Vec<FootprintText>,
    fp_line: Vec<FootprintLine>,
    pads: Vec<Pad>,
    models: Vec<Model>,
}

#[derive(Debug, Default, Clone)]
struct GrText {
    label: String,
    at: (f32, f32),
    layer: String,
    effects: Effects,
}

#[derive(Debug, Default, Clone)]
struct GrCircle {
    center: (f32, f32),
    end: (f32, f32),
    layer: String,
    width: f32,
    tstamp: u32,
}

#[derive(Debug, Default, Clone)]
struct GrArc {
    start: (f32, f32),
    end: (f32, f32),
    angle: f32,
    layer: String,
    width: f32,
    tstamp: u32,
}

#[derive(Debug, Default, Clone)]
struct GrLine {
    start: (f32, f32),
    end: (f32, f32),
    layer: String,
    width: f32,
    tstamp: u32,
}

#[derive(Debug, Default, Clone)]
struct Segment {
    start: (f32, f32),
    end: (f32, f32),
    layer: String,
    width: f32,
    net: u32,
    tstamp: u32,
}

#[derive(Debug, Default, Clone)]
struct Via {
    at: (f32, f32),
    size: f32,
    drill: f32,
    layers: Vec<String>,
    net: u32,
    tstamp: u32,
}

#[derive(Debug, Default, Clone)]
struct Dimension {
    num: u32,
    width: f32,
    layer: String,
    text: GrText,
    feature1: Vec<Point>,
    feature2: Vec<Point>,
    arrow1a: Vec<Point>,
    arrow1b: Vec<Point>,
    arrow2a: Vec<Point>,
    arrow2b: Vec<Point>,
}

#[derive(Debug, Default, Clone)]
struct Polygon {
    points: Vec<Point>,
    filled: bool,
}

#[derive(Debug, Default, Clone)]
struct ZoneFill {
    arc_segments: u8,
    thermal_gap: f32,
    thermal_bridge_width: f32,
    smoothing: String,
    radius: u8,
}

#[derive(Debug, Default, Clone)]
struct Zone {
    net: u32,
    net_name: String,
    layer: String,
    tstamp: u32,
    priority: u8,
    hatch: (String, f32),
    connect_pads: (String, f32), // TODO: fix this
    min_thickness: f32,
    fill_zone: bool,
    fill: ZoneFill,
    polygons: Vec<Polygon>,
}

#[derive(Debug, Default, Clone)]
struct PCB {
    version: u64,
    host: Host,
    general: General,
    page: String,
    layers: Vec<Layer>,
    setup: Setup,
    nets: Vec<Net>,
    net_classes: Vec<NetClass>,
    modules: Vec<Module>,
    dimensions: Vec<Dimension>,
    gr_circles: Vec<GrCircle>,
    gr_texts: Vec<GrText>,
    gr_arcs: Vec<GrArc>,
    gr_lines: Vec<GrLine>,
    segments: Vec<Segment>,
    vias: Vec<Via>,
    zones: Vec<Zone>,
}

fn main() {
    println!("reading test pcb...");

    let contents =
        fs::read_to_string("ferret.kicad_pcb").expect("Something went wrong reading the file");

    let results = lexpr::from_str_custom(&contents, Options::kicad()).unwrap();

    // the pcb structure
    let pcb = results.as_pair().unwrap();
    let iter = pcb.1.list_iter().unwrap();

    let mut pcb = PCB::default();

    for value in iter {
        let v = value.to_vec().unwrap();
        let sym = v.first().unwrap();

        if !sym.is_cons() {
            let name = sym.to_string();
            match name.as_str() {
                "version" => pcb.version = v[1].as_u64().unwrap(),
                "general" => pcb.general = parse_general(v),
                "page" => pcb.page = v[1].as_symbol().unwrap().to_string(),
                "layers" => pcb.layers = parse_layers(v),
                "setup" => println!("setup {:#?}", v[1]),
                "net" => pcb.nets.push(parse_net(v)),
                "net_class" => pcb.net_classes.push(parse_netclass(v)),
                "module" => println!("module {:#?}", v[1]),
                "segment" => pcb.segments.push(parse_segment(v)),
                "via" => pcb.vias.push(parse_via(v)),
                "dimension" => println!("dimension {:#?}", v[1]),
                "gr_circle" => println!("gr_circle {:#?}", v[1]),
                "gr_text" => println!("gr_text {:#?}", v[1]),
                "gr_line" => println!("gr_line {:#?}", v[1]),
                "gr_arc" => println!("gr_arc {:#?}", v[1]),
                "zone" => println!("zone {:#?}", v[1]),
                _ => println!("uwu, what is this? {}", name),
            }
        } else {
            println!("{:#?}", v.to_vec());
        }
    }

    println!("{:#?}", pcb);
}

fn parse_general(v: Vec<lexpr::Value>) -> General {
    let mut g = General::default();
    
    for value in v {
        // first value is a symbol
        if value.is_symbol() {
            continue;
        }

        let param = value.to_vec().unwrap();
        let name = param[0].to_string();

        match name.as_str() {
            "thickness" => g.thickness = param[1].as_f64().unwrap() as f32,
            "drawings" => g.drawings = param[1].as_u64().unwrap() as u32,
            "zones" => g.zones = param[1].as_u64().unwrap() as u32,
            "modules" => g.modules = param[1].as_u64().unwrap() as u32,
            "nets" => g.nets = param[1].as_u64().unwrap() as u32,
            _ => println!("uwu, what is this? {}", name),
        }
    }
    g
}

fn parse_layers(v: Vec<lexpr::Value>) -> Vec<Layer> {
    let mut layers = Vec::new();
    
    for value in v {
        // first value is a symbol
        if value.is_symbol() {
            continue;
        }

        let p = value.to_vec().unwrap();

        let mut l = Layer::default();

        l.num = p[0].as_u64().unwrap() as u32;
        l.name = p[1].as_symbol().unwrap().to_string();
        l.typ = p[2].as_symbol().unwrap().to_string();
        if p.len() == 4 {
            let hidden = p[3].as_symbol().unwrap();
            if hidden == "hide" {
                l.hidden = true
            } else {
                println!("while parsing layer definitions: got {} instead of hide", hidden)
            }
        }

        layers.push(l);
    }

    layers
}

fn parse_net(v: Vec<lexpr::Value>) -> Net {
    let mut net = Net::default();

    net.num = v[1].as_u64().unwrap() as u32;
    if v[2].is_symbol() {
        net.name = v[2].as_symbol().unwrap().to_string();
    } else if v[2].is_string() {
        net.name = v[2].as_str().unwrap().to_string();
    }

    net
}

fn sym_or_str(v: lexpr::Value) -> String {
    if v.is_symbol() {
        v.as_symbol().unwrap().to_string()
    } else if v.is_string() {
        v.as_str().unwrap().to_string()
    } else {
        String::new() // TODO: bugcheck
    }
}

fn parse_netclass(v: Vec<lexpr::Value>) -> NetClass {
    let mut nc = NetClass::default();

    let mut it = v.iter();

    it.next(); // throw away the first element which is just net_class

    nc.name = it.next().unwrap().as_symbol().unwrap().to_string();
    nc.label = it.next().unwrap().as_str().unwrap().to_string();
    
    for value in it{
        let inner = value.to_vec().unwrap();
        let label = sym_or_str(inner[0].clone());

        match label.as_str() {
            "add_net" => nc.nets.push(sym_or_str(inner[1].clone())),
            "clearance" => nc.clearance = inner[1].as_f64().unwrap() as f32,
            "trace_width" => nc.trace_width = inner[1].as_f64().unwrap() as f32,
            "via_dia" => nc.via_dia = inner[1].as_f64().unwrap() as f32,
            "via_drill" => nc.via_drill = inner[1].as_f64().unwrap() as f32,
            "uvia_dia" => nc.uvia_dia = inner[1].as_f64().unwrap() as f32,
            "uvia_drill" => nc.uvia_drill = inner[1].as_f64().unwrap() as f32,
            "diff_pair_width" => nc.diff_pair_width = inner[1].as_f64().unwrap() as f32,
            "diff_pair_gap" => nc.diff_pair_gap = inner[1].as_f64().unwrap() as f32,
            _ => println!("unknown cons in net_class: {}", label),
        }
    }

    nc
}

fn parse_segment(v: Vec<lexpr::Value>) -> Segment {
    let mut seg = Segment::default();

    for value in v.iter(){
        if value.is_symbol() {
            continue;
        }

        let inner = value.to_vec().unwrap();
        let label = sym_or_str(inner[0].clone());

        match label.as_str() {
            "start" => seg.start = (inner[1].as_f64().unwrap() as f32, inner[2].as_f64().unwrap() as f32),
            "end" => seg.end = (inner[1].as_f64().unwrap() as f32, inner[2].as_f64().unwrap() as f32),
            "width" => seg.width = inner[1].as_f64().unwrap() as f32,
            "layer" => seg.layer = sym_or_str(inner[1].clone()),
            "net" => seg.net = inner[1].as_u64().unwrap() as u32,
            "tstamp" => seg.tstamp = inner[1].as_u64().unwrap() as u32,
            _ => println!("unknown cons in net_class: {}", label),
        }
    }

    seg
}

fn parse_via(v: Vec<lexpr::Value>) -> Via {
    let mut via = Via::default();

    for value in v.iter(){
        if value.is_symbol() {
            continue;
        }

        let inner = value.to_vec().unwrap();
        let label = sym_or_str(inner[0].clone());

        match label.as_str() {
            "at" => via.at = (inner[1].as_f64().unwrap() as f32, inner[2].as_f64().unwrap() as f32),
            "size" => via.size = inner[1].as_f64().unwrap() as f32,
            "drill" => via.drill = inner[1].as_f64().unwrap() as f32,
            "layers" => for l in inner[1..].iter() {
                via.layers.push(sym_or_str(l.clone()));
            },
            "net" => via.net = inner[1].as_u64().unwrap() as u32,
            _ => println!("unknown cons in net_class: {}", label),
        }
    }

    via
}
