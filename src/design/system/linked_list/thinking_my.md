# စဉ်းစားချက်- Rust တွင် Linked List Memory စီမံခန့်ခွဲမှု (Thinking: Linked List Memory Management in Rust)

Rust တွင် Linked List များ ရေးသားသည့်အခါ ownership system ၏ တင်းကျပ်သော စည်းမျဉ်းများနှင့် ရင်ဆိုင်ရလေ့ရှိသည်။ မှန်ကန်သော smart pointer ကို ရွေးချယ်ခြင်းသည် list ၏ တည်ဆောက်ပုံ (topology) ပေါ်တွင် မူတည်သည်။

## ၁။ Singly Linked List: ဘာကြောင့် `Box<T>` ကို သုံးရသလဲ?
Singly linked list သည် **recursive ဖြစ်ပြီး single-ownership (ပိုင်ရှင်တစ်ဦးတည်း)** ရှိသော တည်ဆောက်ပုံမျိုးဖြစ်သည်။
- **`Box<T>`**: တစ်ဦးတည်းပိုင်ဆိုင်မှုကို ကိုယ်စားပြုသည်။ node တစ်ခုစီသည် ၎င်းနောက်ရှိ node ကို ပိုင်ဆိုင်သည်။
- **ဘာကြောင့် `Rc` မသုံးတာလဲ?**: ရိုးရှင်းသော singly linked list တွင် node တစ်ခုကို နေရာနှစ်ခုမှ မျှဝေသုံးစွဲခြင်း (shared ownership) သို့မဟုတ် နောက်ပြန်ညွှန်းခြင်း (back-references) များ မရှိသောကြောင့် reference counting ၏ overhead (ဝန်ထုပ်ဝန်ပိုး) မလိုအပ်ပါ။ `Box` သည် heap allocation အတွက် အထိရောက်ဆုံး ရွေးချယ်မှုဖြစ်သည်။

## ၂။ Doubly Linked List: ဘာကြောင့် `Rc<RefCell<T>>` နှင့် `Weak<T>` ကို သုံးရသလဲ?
Doubly linked list သည် **shared ownership (မျှဝေပိုင်ဆိုင်မှု)** နှင့် **bidirectional references (နှစ်ဖက်သွားလမ်းကြောင်းများ)** လိုအပ်သည်။
- **`Rc<T>` (Reference Counted)**: Node တစ်ခုကို ရှေ့က node (`next` မှတစ်ဆင့်) နှင့် list ၏ `head`/`tail` pointer များမှ တစ်ပြိုင်နက် ပိုင်ဆိုင်ခွင့်ပေးသည်။
- **`RefCell<T>`**: **Interior mutability** ကို ပေးသည်။ `Rc` သည် immutable reference သာ ပေးသောကြောင့် runtime တွင် `next`/`prev` pointer များကို ပြောင်းလဲရန် (mutate) `RefCell` လိုအပ်သည်။
- **`Weak<T>`**: `prev` pointer အတွက် အလွန်အရေးကြီးသည်။ အကယ်၍ `prev` တွင် `Rc` ကို သုံးပါက **reference cycle** ဖြစ်ပေါ်ပြီး (A -> B နှင့် B -> A) memory leak ဖြစ်စေပါမည် (memory မှ ဘယ်တော့မှ ပယ်ဖျက်မည်မဟုတ်ပါ)။ `Weak` သည် reference count ကို မတိုးစေဘဲ ထို cycle ကို ချိုးဖျက်ပေးသည်။

## နှိုင်းယှဉ်ချက် အကျဉ်းချုပ် (Comparison Summary)

| Feature | `Box<T>` | `Rc<RefCell<T>>` |
| :--- | :--- | :--- |
| **Ownership** | တစ်ဦးတည်းပိုင် (Unique) | မျှဝေပိုင် (Shared) |
| **Overhead** | အလွန်နည်းပါးသည် | များသည် (Control block + check) |
| **Flexibility** | နည်းပါးသည် | မြင့်မားသည် |
| **Best For** | Singly Linked, Trees | Doubly Linked, Graphs |

## ၃။ အဆင့်မြင့် ပုံစံများ (Advanced Patterns)

### Arena-based (Index-backed)
Pointer များအစား node များကို `Vec<Node>` ထဲတွင် သိမ်းဆည်းပြီး `usize` index များကို သုံးခြင်း။
- **ဘာကြောင့်လဲ?**: `RefCell` ၏ ဝန်ထုပ်ဝန်ပိုးနှင့် borrow checker ၏ ရှုပ်ထွေးမှုများကို ရှောင်ရှားနိုင်သည်။ Cache-friendly ဖြစ်ပြီး ရှုပ်ထွေးသော graph များအတွက် ကောင်းမွန်သည်။

### Raw Pointers (`unsafe`)
`*mut T` ကို သုံးသော "C-style" နည်းလမ်း။
- **ဘာကြောင့်လဲ?**: `std::collections::LinkedList` တွင် ဤနည်းကို သုံးသည်။ အမြန်ဆုံး performance ရရှိသော်လည်း `unsafe` block များ သုံးရန်လိုအပ်ပြီး memory ကို ကိုယ်တိုင်စီမံခန့်ခွဲရမည်။

### Functional (Immutable)
Node များကို `Rc<T>` ဖြင့် ပတ်ထားသော်လည်း ပြောင်းလဲ၍မရ (immutable) ပါ။
- **ဘာကြောင့်လဲ?**: Functional programming များတွင် သုံးသည်။ List အများအပြားသည် တူညီသော နောက်ပိုင်း node (tail) များကို ဘေးကင်းစွာ မျှဝေသုံးစွဲနိုင်စေသည်။
