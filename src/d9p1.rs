use std::io;

type Disk = Vec<Option<i64>>;

// Visualize the disk state like in the problem description e.g. "0..111....22222"
fn _disk_to_string(disk: &Disk) -> String {
    return disk
        .iter()
        .map(|&x| match x {
            None => ".".to_owned(),
            Some(fid) => fid.to_string(),
        })
        .collect();
}

fn expand_disk(s: &str) -> Disk {
    let mut disk: Disk = Disk::new();
    let mut free = false;
    let mut fid = 0;
    for c in s.chars() {
        let content = if free { None } else { Some(fid) };
        let len = c.to_digit(10).unwrap() as usize;
        disk.extend(vec![content; len]);
        if !free {
            fid += 1;
        }
        free = !free;
    }
    return disk;
}

fn compact(disk: &mut Disk) {
    let mut i = 0;
    let mut j = disk.len() - 1;
    loop {
        // Find leftmost free space
        while i < j && disk[i].is_some() {
            i += 1;
        }
        if i >= j {
            break;
        }
        // Move rightmost file block into the free space
        disk[i] = disk[j];
        disk[j] = None;
        j -= 1;
    }
}

fn checksum(disk: &Disk) -> i64 {
    let mut ans: i64 = 0;
    for (i, fid) in disk.iter().enumerate() {
        ans += (i as i64) * fid.unwrap_or(0);
    }
    return ans;
}

fn main() {
    for line in io::stdin().lines() {
        let mut disk = expand_disk(&line.unwrap());
        compact(&mut disk);
        println!("{}", checksum(&disk));
    }
}
