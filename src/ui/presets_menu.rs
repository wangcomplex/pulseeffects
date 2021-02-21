use gtk::prelude::*;
use gtk::subclass::widget::WidgetImplExt;
use gtk::CompositeTemplate;

use crate::presets::manager;
use std::sync::{Arc, Mutex};

mod imp {
    use super::*;
    use glib::subclass;
    use gtk::subclass::prelude::*;

    #[derive(Debug, CompositeTemplate)]
    #[template(file = "presets_menu.ui")]
    pub struct ExPresetsMenu {
        #[template_child]
        pub stack: TemplateChild<gtk::Stack>,

        #[template_child]
        pub import_output: TemplateChild<gtk::Button>,

        #[template_child]
        pub add_output: TemplateChild<gtk::Button>,

        #[template_child]
        pub output_name: TemplateChild<gtk::Entry>,

        #[template_child]
        pub output_scrolled_window: TemplateChild<gtk::ScrolledWindow>,

        #[template_child]
        pub output_listbox: TemplateChild<gtk::ListBox>,

        #[template_child]
        pub import_input: TemplateChild<gtk::Button>,

        #[template_child]
        pub add_input: TemplateChild<gtk::Button>,

        #[template_child]
        pub input_name: TemplateChild<gtk::Entry>,

        #[template_child]
        pub input_scrolled_window: TemplateChild<gtk::ScrolledWindow>,

        #[template_child]
        pub input_listbox: TemplateChild<gtk::ListBox>,

        pub presets_manager: manager::Manager,
    }

    impl ObjectSubclass for ExPresetsMenu {
        const NAME: &'static str = "ExPresetsMenu";
        type Type = super::ExPresetsMenu;
        type ParentType = gtk::Popover;
        type Interfaces = ();
        type Instance = subclass::simple::InstanceStruct<Self>;
        type Class = subclass::simple::ClassStruct<Self>;

        glib::object_subclass!();

        fn new() -> Self {
            Self {
                stack: TemplateChild::default(),
                import_output: TemplateChild::default(),
                add_output: TemplateChild::default(),
                output_name: TemplateChild::default(),
                output_scrolled_window: TemplateChild::default(),
                output_listbox: TemplateChild::default(),
                import_input: TemplateChild::default(),
                add_input: TemplateChild::default(),
                input_name: TemplateChild::default(),
                input_scrolled_window: TemplateChild::default(),
                input_listbox: TemplateChild::default(),
                presets_manager: manager::Manager::new(),
            }
        }

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self::Type>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for ExPresetsMenu {
        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(obj);

            self.output_listbox
                .set_sort_func(Some(Box::new(on_listbox_sort)));
            self.input_listbox
                .set_sort_func(Some(Box::new(on_listbox_sort)));
        }
    }

    impl WidgetImpl for ExPresetsMenu {
        fn show(&self, widget: &Self::Type) {
            self.parent_show(widget);

            println!("oi");

            let presets_manager = Arc::new(Mutex::new(&self.presets_manager));

            {
                let presets_manager = presets_manager.clone();
                let input_listbox = self.input_listbox.clone();
                let output_listbox = self.output_listbox.clone();

                self.populate_listbox(&manager::PresetType::Input);

                // populate_listbox(
                //     &presets_manager,
                //     &manager::PresetType::Input,
                //     &input_listbox,
                // );
            }
        }
    }

    impl PopoverImpl for ExPresetsMenu {}

    impl ExPresetsMenu {
        pub fn populate_listbox(&self, preset_type: &manager::PresetType) {
            let names = self.presets_manager.get_names(preset_type);

            for name in names {}
        }
    }
}

glib::wrapper! {
    pub struct ExPresetsMenu(ObjectSubclass<imp::ExPresetsMenu>) @extends gtk::Widget, gtk::Popover;
}

impl ExPresetsMenu {
    pub fn new() -> Self {
        glib::Object::new(&[]).expect("Failed to create the presets menu")
    }
}

// pub fn build_ui(button: &gtk::Button) -> gtk::Grid {

//         button.connect_clicked(move |obj| {
//             let top_widget = obj
//                 .get_toplevel()
//                 .expect("Could not get presets menu top level widget");

//             let height = top_widget.get_allocated_height() as f32;

//             output_scrolled_window.set_max_content_height((0.7 * height) as i32);

//             populate_listbox(
//                 &presets_manager,
//                 &manager::PresetType::Output,
//                 &output_listbox,
//             );
//         });
//     }

//     {
//         let output_name = resources.output_name.clone();
//         let output_listbox = resources.output_listbox.clone();
//         let presets_manager = presets_manager.clone();

//         resources.add_output.connect_clicked(move |_btn| {
//             create_preset(
//                 &presets_manager,
//                 &manager::PresetType::Output,
//                 &output_name,
//                 &output_listbox,
//             );

//             populate_listbox(
//                 &presets_manager,
//                 &manager::PresetType::Output,
//                 &output_listbox,
//             );
//         });
//     }

//     {
//         let input_name = resources.input_name.clone();
//         let input_listbox = resources.input_listbox.clone();
//         let presets_manager = presets_manager.clone();

//         resources.add_input.connect_clicked(move |_btn| {
//             create_preset(
//                 &presets_manager,
//                 &manager::PresetType::Input,
//                 &input_name,
//                 &input_listbox,
//             );

//             populate_listbox(
//                 &presets_manager,
//                 &manager::PresetType::Input,
//                 &input_listbox,
//             );
//         });
//     }

//     return resources.widgets_grid;
// }

fn create_preset(
    presets_manager: &std::sync::Arc<std::sync::Mutex<manager::Manager>>,
    preset_type: &manager::PresetType,
    entry: &gtk::Entry,
    listbox: &gtk::ListBox,
) {
    let name = entry.get_text().to_string();

    if name.chars().all(char::is_alphanumeric) {
        presets_manager.lock().unwrap().add(preset_type, &name);

        populate_listbox(&presets_manager, &preset_type, &listbox);
    }

    entry.set_text("");
}

fn on_listbox_sort(row1: &gtk::ListBoxRow, row2: &gtk::ListBoxRow) -> i32 {
    let mut names = Vec::new();
    let name1 = row1.get_widget_name();
    let name2 = row2.get_widget_name();

    names.push(&name1);
    names.push(&name2);
    names.sort();

    if name1 == *names[0] {
        return -1;
    }
    if name2 == *names[0] {
        return 1;
    }

    return 0;
}

fn populate_listbox(
    presets_manager: &std::sync::Arc<std::sync::Mutex<manager::Manager>>,
    preset_type: &manager::PresetType,
    listbox: &gtk::ListBox,
) {
    // let children = listbox.get_children();

    // for child in children {
    //     listbox.remove(&child);
    // }

    let names = presets_manager.lock().unwrap().get_names(preset_type);

    for name in names {
        let builder =
            gtk::Builder::from_resource("/com/github/wwmm/pulseeffects/ui/preset_row.glade");

        let row: gtk::ListBoxRow = builder
            .get_object("preset_row")
            .expect("builder could not get the widget: preset_row");

        let apply_btn: gtk::Button = builder
            .get_object("apply")
            .expect("builder could not get the widget: apply");

        let save_btn: gtk::Button = builder
            .get_object("save")
            .expect("builder could not get the widget: save");

        let remove_btn: gtk::Button = builder
            .get_object("remove")
            .expect("builder could not get the widget: remove");

        let label: gtk::Label = builder
            .get_object("name")
            .expect("builder could not get the widget: name");

        let autoload_btn: gtk::Button = builder
            .get_object("autoload")
            .expect("builder could not get the widget: autoload");

        row.set_widget_name(name.as_str());

        label.set_text(name.as_str());

        // if (is_autoloaded(preset_type, name)) {
        //     autoload_btn->set_active(true);
        // }

        let presets_manager = presets_manager.clone();

        {
            let presets_manager = presets_manager.clone();
            let preset_type = (*preset_type).clone();
            let name = name.clone();

            apply_btn.connect_clicked(move |_btn| {
                // settings->set_string("last-used-preset", row->get_name());

                presets_manager.lock().unwrap().load(&preset_type, &name);
            });
        }

        {
            let presets_manager = presets_manager.clone();
            let preset_type = (*preset_type).clone();
            let name = name.clone();

            save_btn.connect_clicked(move |_btn| {
                presets_manager.lock().unwrap().save(&preset_type, &name);
            });
        }

        {
            let presets_manager = presets_manager.clone();
            let preset_type = (*preset_type).clone();
            let name = name.clone();
            let listbox = listbox.clone();

            remove_btn.connect_clicked(move |_btn| {
                presets_manager.lock().unwrap().remove(&preset_type, &name);

                populate_listbox(&presets_manager, &preset_type, &listbox);
            });
        }

        autoload_btn.connect_clicked(|obj| {
            // if (preset_type == PresetType::output) {
            //     auto dev_name = build_device_name(preset_type, app->pm->server_info.default_sink_name);
            //     if (autoload_btn->get_active()) {
            //       app->presets_manager->add_autoload(dev_name, name);
            //     } else {
            //       app->presets_manager->remove_autoload(dev_name, name);
            //     }
            //   } else {
            //     auto dev_name = build_device_name(preset_type, app->pm->server_info.default_source_name);
            //     if (autoload_btn->get_active()) {
            //       app->presets_manager->add_autoload(dev_name, name);
            //     } else {
            //       app->presets_manager->remove_autoload(dev_name, name);
            //     }
            //   }
            //   populate_listbox(preset_type);
        });

        // listbox.add(&row);
        // listbox.show_all();
    }
}