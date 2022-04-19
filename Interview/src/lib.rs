use rand::{self, Rng};


/// 将员工均匀分成k组，每组人数 people_num / k 到 (people_num / k) + 1
/// 对每个员工模拟选择分组，分组人数达到上限就线性往后选择分组
/// 时间复杂度O(n)，
pub fn rand_group(arr:&Vec<String>,k:usize)->Vec<Vec<String>>{
    if k == 0{
        return vec![arr.clone()];
    }
    if arr.len() == 0{
        return vec![];
    }
    let mut group = vec![vec![];k];
    let people_num = arr.len();
    let num_team = if people_num % k == 0{
        people_num/k
    }else{
        people_num/k + 1
    };
    let mut rng = rand::thread_rng();


    for idx in 0..k*(num_team-1){
        let mut r = rng.gen_range(0..k);
        while group[r].len()>= num_team-1{
            r+=1;
            r%=k;
        }
        group[r].push(arr[idx].clone());
    }

    for idx in k*(num_team-1)..people_num{
        let mut r = rng.gen_range(0..k);
        while group[r].len()>= num_team{
            r+=1;
            r%=k;
        }
        group[r].push(arr[idx].clone());
    }

    group
}

/// 模拟分组选择员工
/// 时间复杂度O(n),n次选择和交换
fn rand_group_select(arr:&Vec<String>,k:usize)->Vec<Vec<String>>{
    if k == 0{
        return vec![arr.clone()];
    }
    if arr.len() == 0{
        return vec![];
    }
    let mut ans = vec![vec![];k];
    let mut plist = arr.clone();
    let mut tail = arr.len();
    let mut rng = rand::thread_rng();
    for i in 0..plist.len(){
        let r = rng.gen_range(0..tail);
        tail-=1;
        ans[i%k].push(plist[r].clone());
        plist.swap(r, tail);
    }


    ans
}


#[cfg(test)]
mod tests {
    use std::fmt::format;

    use crate::{rand_group,rand_group_select};

    #[test]
    fn test_range_group(){

        let mut arr = vec![];
        let people_num = 200;
        for i in 0..people_num{
            arr.push(gen_rand_string());
        }
        
        {// 可以整除的情况   
            let k = 20;
            let group = rand_group(&arr, k);
            let num_team = people_num/k;
            for v in group{
                assert!(v.len()==num_team);
            }
        }

        {// 无法整除的情况
            let k = 19;
            let group = rand_group(&arr, k);
            let num_team = people_num/k;
            for v in group{
                assert!((v.len()-num_team) <= 1);
            }
        }
        {// 分组数为0的情况
            let k = 0;
            let group = rand_group(&arr, k);
            assert!(group.len() == 1 && group[0].len() == arr.len());
        }
        {// 人数为0的情况
            let k = 18;
            let arr = vec![];
            let group = rand_group(&arr, k);
            for v in group{
                assert!(v.len()==0);
            }
        }

    }

    #[test]
    fn test_range_group_select(){
        let mut arr = vec![];
        let people_num = 200;
        for _ in 0..people_num{
            arr.push(gen_rand_string());
        }
        
        {// 可以整除的情况   
            let k = 20;
            let group = rand_group_select(&arr, k);
            let num_team = people_num/k;
            for v in group{
                assert!(v.len()==num_team);
            }
        }

        {// 无法整除的情况
            let k = 19;
            let group = rand_group_select(&arr, k);
            let num_team = people_num/k;
            for v in group{
                assert!((v.len()-num_team) <= 1);
            }
        }
        {// 分组数为0的情况
            let k = 0;
            let group = rand_group_select(&arr, k);
            assert!(group.len() == 1 && group[0].len() == arr.len());
        }
        {// 人数为0的情况
            let k = 18;
            let arr = vec![];
            let group = rand_group_select(&arr, k);
            for v in group{
                assert!(v.len()==0);
            }
        }
    }

    fn gen_rand_string()->String{
        let r:usize = rand::random();
        format!("uid:{}",r)
    }
}
