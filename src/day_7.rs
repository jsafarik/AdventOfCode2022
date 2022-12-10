pub fn main() {
    parse_input(include_str!("resources/day_7_file_system.txt"));
}

fn parse_input(input: &str) {
    let file_system = &mut FileSystem::new();
    for line in input.lines() {
        if line.starts_with("$ cd") {
            if line.ends_with("..") {
                file_system.parent_one_up();
            } else if line.contains("cd /") {
                file_system.add_file("/", 0, true);
            } else {
                file_system.set_child_as_parent(line.split_whitespace().last().unwrap());
            }
        } else if line.starts_with("dir") {
            let parts = line.split_whitespace().collect::<Vec<&str>>();
            file_system.add_file(parts.get(1).unwrap(), 0, true);
        } else if !line.starts_with("$") {
            let parts = line.split_whitespace().collect::<Vec<&str>>();
            file_system.add_file(parts.get(1).unwrap(), parts.get(0).unwrap().parse::<usize>().unwrap(), false);
        }
    }

    let dirs = file_system.get_all_directories();

    let (_, root_size) = dirs.first().unwrap();
    let total_space: usize = 70_000_000;
    let needed_free_space: usize = 30_000_000;
    let to_be_deleted = needed_free_space - (total_space - root_size);
    println!("Root size: {}", root_size);
    println!("Total size: {}", total_space);
    println!("Needed space: {}", needed_free_space);
    println!("{} needs to be deleted", to_be_deleted);

    let sum = dirs
        .iter()
        .filter(|(_, size)| *size <= 100000)
        .fold(0, |acc, (_, size)| acc + size);
    println!("Sum of directories with at max 100000 in size: {}", sum);

    let mut dirs_sorted = dirs.clone();
    dirs_sorted.sort_by(|(_, dir_1), (_, dir_2)| dir_1.cmp(dir_2));
    let (_, dir_to_be_deleted) = dirs_sorted.iter().find(|(_, size)| *size > to_be_deleted).unwrap();
    println!("Dir with size {} to be deleted", dir_to_be_deleted);
}

struct FileSystem {
    files: Vec<File>,
    current_parent: usize,
}

impl FileSystem {
    fn new() -> FileSystem {
        FileSystem {
            files: Vec::new(),
            current_parent: 0,
        }
    }

    fn get_file(&self, idx: usize) -> &File {
        self.files.get(idx).unwrap()
    }

    fn add_file(&mut self, name: &str, size: usize, is_directory: bool) {
        println!("Adding file {} to {}", name, self.current_parent);

        let index = self.files.len();

        self.files.push(File {
            name: name.to_string(),
            size,
            parent: self.current_parent,
            children: Vec::new(),
            index,
            is_directory,
        });

        let parent = self.files.get_mut(self.current_parent).unwrap();
        if index != 0 {
            parent.children.push(index);
        }
    }

    fn parent_one_up(&mut self) {
        self.current_parent = self.files.get(self.current_parent).unwrap().parent
    }

    fn set_child_as_parent(&mut self, name: &str) {
        self.current_parent = self
            .files
            .get(self.current_parent)
            .unwrap()
            .children
            .iter()
            .find(|child| self.files.get(**child).unwrap().name == name)
            .unwrap()
            .clone();
    }

    fn find_file(&self, name: &str) -> &File {
        self.files.iter().find(|file| file.name.as_str() == name).unwrap()
    }

    fn is_dir(&self, index: usize) -> bool {
        self.files.get(index).unwrap().size == 0
    }

    fn print(&self, idx: usize) {
        let parent = self.files
            .get(idx)
            .unwrap();
        println!("Parent: {} ({}): {}", parent.name, "Dir", parent.size);

        parent
            .children
            .iter()
            .for_each(|child| {
                let actual_child = self.files.get(*child).unwrap();
                println!("{} ({}): {} (parent is {})", actual_child.name, if actual_child.is_directory { "Dir" } else { "File" }, actual_child.size, actual_child.parent)
            })
    }

    fn calculate_dir_size(&self, idx: usize) -> usize {
        let mut size: usize = 0;
        for file in self.files.get(idx).unwrap().children.clone() {
            if self.files.get(file).unwrap().is_directory {
                size += self.calculate_dir_size(file);
            } else {
                size += self.files.get(file).unwrap().size;
            }
        }
        size
    }

    fn get_all_directories(&self) -> Vec<(&str, usize)> {
        let mut result: Vec<(&str, usize)> = Vec::new();
        for file in 0..self.files.len() {
            if self.files.get(file).unwrap().is_directory {
                result.push((self.files.get(file).unwrap().name.as_str(), self.calculate_dir_size(self.files.get(file).unwrap().index)));
            }
        }
        result
    }
}

struct File {
    name: String,
    size: usize,
    parent: usize,
    children: Vec<usize>,
    index: usize,
    is_directory: bool,
}


