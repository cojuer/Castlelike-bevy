pub struct ItemStack {
    item_kind: usize,
    amount: usize,
    max_amount: usize,
}

impl ItemKind for ItemStack {
    type KindType = usize;

    fn item_kind(&self) -> Self::KindType {
        self.item_kind
    }
}

impl Stackable for ItemStack {
    fn amount(&self) -> usize {
        self.amount
    }
    fn max_amount(&self) -> usize {
        self.max_amount
    }

    fn set_amount(&mut self, amount: usize) -> Result<(), ()> {
        if amount > self.max_amount {
            return Err(());
        } else {
            self.amount = amount;
            return Ok(());
        }
    }

    fn add_amount(&mut self, amount: usize) -> usize {
        if amount > self.max_amount - self.amount {
            let res = amount - (self.max_amount - self.amount);
            self.amount = self.max_amount;
            return res;
        } else {
            self.amount += amount;
            return 0;
        }
    }
}

/// trait for unique identification
pub trait Id {
    type IdType;

    fn id(&self) -> Self::IdType;
}

/// trait for unique identification of same items
pub trait ItemKind {
    type KindType: Eq;

    fn item_kind(&self) -> Self::KindType;
}

pub trait Stackable {
    fn amount(&self) -> usize;
    fn max_amount(&self) -> usize;

    /// set new amount, checks whether <= max amount
    fn set_amount(&mut self, amount: usize) -> Result<(), ()>;

    /// increase amount of self, return amount above max if any
    fn add_amount(&mut self, amount: usize) -> usize;
}

/// Representation of in-game container
/// - consists of ordered slots;
/// - limited size;
/// - slot can store multiple items of the same kind;
pub struct Container<T>
where
    T: Stackable + ItemKind,
{
    slots: Vec<Option<T>>,
}

impl<T: Stackable + ItemKind> Container<T> {
    pub fn new(size: usize) -> Self {
        Self {
            slots: std::iter::repeat_with(|| None).take(size).collect(),
        }
    }

    /// try to add item to the container
    /// return remaining items
    fn add_item(&mut self, item: T) -> Option<T> {
        let mut rem_item = Some(item);
        for index in 0..self.slots.len() {
            if let Some(item) = rem_item {
                rem_item = self.add_to_slot(item, index);
            } else {
                return None;
            }
        }
        return rem_item;
    }

    /// try to add item to concrete slot of container
    /// return remaining items
    fn add_to_slot(&mut self, mut item: T, index: usize) -> Option<T> {
        match &mut self.slots[index] {
            Some(slot_item) => {
                if slot_item.item_kind() == item.item_kind() {
                    // unwrap as remaining amount is valid
                    item.set_amount(slot_item.add_amount(item.amount()))
                        .unwrap();
                    if item.amount() == 0 {
                        return None;
                    }
                }
                return Some(item);
            }
            None => {
                self.slots[index] = Some(item);
                return None;
            }
        }
    }

    fn erase_slot(&mut self, index: usize) {
        self.slots[index] = None;
    }

    fn swap_slots(&mut self, index_a: usize, index_b: usize) {
        self.slots.swap(index_a, index_b);
    }

    fn slot(&self, index: usize) -> Option<&T> {
        self.slots[index].as_ref()
    }

    fn slot_mut(&mut self, index: usize) -> Option<&mut T> {
        self.slots[index].as_mut()
    }

    fn take(&mut self, index: usize) -> Option<T> {
        let mut slot_contents = None;
        std::mem::swap(&mut self.slots[index], &mut slot_contents);
        slot_contents
    }

    fn size(&self) -> usize {
        self.slots.len()
    }
}

#[test]
fn test_container_add_and_get() {
    let mut cont = Container::<ItemStack>::new(1);
    let item = ItemStack {
        item_kind: 0,
        amount: 1,
        max_amount: 2,
    };
    cont.add_item(item);
    assert!(
        cont.slot(0).is_some()
            && cont.slot(0).unwrap().item_kind() == 0
            && cont.slot_mut(0).is_some()
            && cont.slot_mut(0).unwrap().item_kind() == 0
    );
}

#[test]
fn test_container_take_removes_item() {
    let mut cont = Container::<ItemStack>::new(1);
    let item = ItemStack {
        item_kind: 0,
        amount: 1,
        max_amount: 2,
    };
    cont.add_item(item);
    assert!(cont.take(0).is_some() && cont.slot(0).is_none());
}

#[test]
fn test_container_can_not_add_more_unique_items_than_slots() {
    let mut cont = Container::<ItemStack>::new(1);
    let item1 = ItemStack {
        item_kind: 0,
        amount: 1,
        max_amount: 2,
    };
    cont.add_item(item1);
    let item2 = ItemStack {
        item_kind: 1,
        amount: 1,
        max_amount: 2,
    };
    assert!(cont.add_item(item2).is_some());
}

#[test]
fn test_container_add_multiple_items_to_slot() {
    let mut cont = Container::<ItemStack>::new(1);
    let item1 = ItemStack {
        item_kind: 0,
        amount: 1,
        max_amount: 2,
    };
    cont.add_item(item1);
    let item2 = ItemStack {
        item_kind: 0,
        amount: 1,
        max_amount: 2,
    };
    assert!(cont.add_item(item2).is_none());
    assert_eq!(cont.slot(0).unwrap().amount(), 2);
}

#[test]
fn test_container_items_fill_non_full_slots() {
    let mut cont = Container::<ItemStack>::new(3);
    let item1 = ItemStack {
        item_kind: 0,
        amount: 2,
        max_amount: 3,
    };
    let item2 = ItemStack {
        item_kind: 0,
        amount: 2,
        max_amount: 3,
    };
    cont.add_item(item1);
    cont.add_item(item2);
    let item3 = ItemStack {
        item_kind: 0,
        amount: 3,
        max_amount: 3,
    };
    assert!(cont.add_item(item3).is_none());
    assert_eq!(cont.slot(0).unwrap().amount(), 3);
    assert_eq!(cont.slot(1).unwrap().amount(), 3);
    assert_eq!(cont.slot(2).unwrap().amount(), 1);
}
