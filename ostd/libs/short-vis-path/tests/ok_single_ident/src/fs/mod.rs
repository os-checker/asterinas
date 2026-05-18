mod nested;
mod procfs;

mod mbe {
    #![short_vis_path::add(fs)]
    pub(in fs) macro mbe() {}
}

mod ramfs {
    mod fs {
        #![short_vis_path::add(fs)]

        pub(in fs) fn shallow_fs_wins() {}
    }
}

mod tmpfs {
    mod fs;
}
