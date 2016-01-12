use std::cell::Cell;
use std::cell::RefCell;

use super::time;

struct ProfilerData{
    name: &'static str,
    parent: i32,
    parent_count: u8,
    //TODO: change to ns
    total_time: f64,
    max_time: f64,
    min_time: f64,
    times_called: u32,
}

struct ProfilerManData{
    last_profile: Cell<i32>,
    profile_data: RefCell<Vec<ProfilerData>>,
}

thread_local!(static PROFILER_DATA: ProfilerManData = ProfilerManData{
    last_profile: Cell::new(-1),
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
                index = data.len();
                let parent = d.last_profile.get();
                println!("{}",parent);
                println!("{}",index);
                d.last_profile.set(index as i32);
                let parent_count = if parent !=  -1 {
                    data[parent as usize].parent_count+1
                }else{
                    0
                };
                data.push(ProfilerData{
                    name: name,
                    parent: parent,
                    parent_count: parent_count,
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
            println!("Total|  #  | Avg   | Min   | Max   : Name");
            for e in data.iter(){
                println!("{:.*} | {:.*} | {:.*} | {:.*} | {:.*} : {}{}"
                         ,2,e.total_time * 1000.0
                         ,3,e.times_called
                         ,3,e.total_time  / (e.times_called as f64) * 1000.0
                         ,3,e.min_time * 1000.0
                         ,3,e.max_time * 1000.0
                         ,(0..e.parent_count).map(|_|' ').collect::<String>()
                         ,e.name);
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
