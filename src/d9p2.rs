use std::io;

type Disk = Vec<Option<usize>>;

struct Block {
    start: usize,
    len: usize,
}

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
    let mut fid: usize = 0;

    let mut empty: Vec<Block> = Vec::new();
    let mut files: Vec<Block> = Vec::new();
    for c in s.chars() {
        let content = if free { None } else { Some(fid) };
        let len = c.to_digit(10).unwrap() as usize;
        let pos = Block {
            start: disk.len(),
            len,
        };
        if free {
            empty.push(pos);
        } else {
            files.push(pos);
            fid += 1;
        }
        disk.extend(vec![content; len]);
        free = !free;
    }

    // dbg!(_disk_to_string(&disk));

    // Compact
    for (fid, f) in files.iter().enumerate().rev() {
        for e in &mut empty {
            if e.start < f.start && e.len >= f.len {
                // Update disk
                for i in 0..f.len {
                    disk[e.start + i] = Some(fid);
                    disk[f.start + i] = None;
                }
                // Update empty block
                e.start += f.len;
                e.len -= f.len;
                break;
            }
        }
    }

    // dbg!(_disk_to_string(&disk));

    return disk;
}

fn checksum(disk: &Disk) -> usize {
    let mut ans: usize = 0;
    for (i, fid) in disk.iter().enumerate() {
        ans += (i as usize) * fid.unwrap_or(0);
    }
    return ans;
}

fn main() {
    for line in io::stdin().lines() {
        let disk = expand_disk(&line.unwrap());
        println!("{}", checksum(&disk));
    }
}
