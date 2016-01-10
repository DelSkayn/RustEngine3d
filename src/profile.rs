use std::cell::Cell;
use std::cell::RefCell;

use super::time;

struct ProfilerData{
    name: &'static str,
    parent: usize,
    //TODO: change to ns
    total_time: f64,
    max_time: f64,
    min_time: f64,
    times_called: u32,
}

struct ProfilerManData{
    last_profile: Cell<usize>,
    profile_data: RefCell<Vec<ProfilerData>>,
}

thread_local!(static PROFILER_DATA: ProfilerManData = ProfilerManData{
    last_profile: Cell::new(0),
    profile_data: RefCell::new(Vec::new()),
});

pub struct ProfileSample{
    start_time: f64,
    index: usize,
}

impl ProfileSample{
    pub fn new(name: &'static str) -> Self{
        let mut index = 0;
        PROFILER_DATA.with(|d|{
            let mut found = false;
            let mut data = d.profile_data.borrow_mut();
            //check if there is still one availeble
            for i in 0..data.len(){
                if data[i].name == name {
                    found = true;
                    index = i;
                }
            }
            if !found {
                let parent = d.last_profile.get();
                d.last_profile.set(index);
                data.push(ProfilerData{
                    name: name,
                    parent: parent,
                    total_time: 0.0,
                    times_called: 0,
                    max_time: 0.0,
                    min_time: 1000.0,
                });
            }
        });
        ProfileSample{
            start_time: time::precise_time_s(),
            index: index,
        }
    }

    pub fn print(){
        PROFILER_DATA.with(|d|{
            let data = d.profile_data.borrow_mut();
            println!("Profile: ");
            for e in data.iter(){
            println!("{} || total:{} | times:{} | avg:{} | min:{} | max:{} "
                     ,e.name
                     ,e.total_time * 1000.0
                     ,e.times_called
                     ,e.total_time  / (e.times_called as f64) * 1000.0
                     ,e.min_time * 1000.0
                     ,e.max_time * 1000.0);
            }
        });
    }
    pub fn clear(){
        PROFILER_DATA.with(|d|{
            let mut data = d.profile_data.borrow_mut();
            data.clear();
            d.last_profile.set(0);
        });
    }
}

impl Drop for ProfileSample{
    fn drop(&mut self){
        let time = time::precise_time_s() - self.start_time;

        PROFILER_DATA.with(|d|{
            let mut data = d.profile_data.borrow_mut();
            data[self.index].total_time += time;
            data[self.index].times_called += 1;

            if data[self.index].min_time > time {
                data[self.index].min_time = time;
            };

            if data[self.index].max_time < time {
                data[self.index].max_time = time;
            };

            d.last_profile.set(data[self.index].parent);
        });
    }
}
