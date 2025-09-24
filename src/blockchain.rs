pub fn create_blockchain(genesis_address:&str)->Blockchain{
    let db = sled::open(current_dir().unwrap().join("data")).unwrap();
    let blocks_tree = db.open_tree(BLOCKS_TREE).unwrap(); 
    let data = blocks_tree.get(TIP_BLOCK_HASH_KEY).unwrap();
    let tip_hash;

    if data.is_none(){
        let coinbase_tx = Transaction::new_coinbase_tx(genesis_address);
        let block = Block::generate_genesis_block(&coinbase_tx);
        Self::update_blocks_tree(&blocks_tree,&block);
        tip_hash = String::from(block.get_hash());
    }else{
        tip_hash = String::from_utf8(data.unwrap().to_vec()).unwrap());
    }
    Blockchain{
        tip_hash: Arc::new(RwLock::new(tip_hash)),
        db, 
    } 

}

pub fn new_blockchain()->Blockchain{
    let db = sled::open(current_dir().unwrap().join("data")).unwrap();
    let blocks_tree = db.open_tree(BLOCKS_TREE).unwrap(); 
    let tip_bytes = blocks_tree.get(TIP_BLOCK_HASH_KEY).unwrap()
    .expect("No existe un blockcain. Create one first.");
    Blockchain{
        tip_hash: Arc::new(RwLock::new(tip_hash)),
        db, 
    }
}
pub fn get_db(&self)-> &Db {
    &self.db
}

pub fn get_tip_hash(&self)->String{
    self.tip_hash.read().unwrap().clone()
}

pub fn set_tip_hash(&self,new_tip_hash &str){
    let mut tip_hash = self.tip_hash.write().unwrap();
    *tip_hash = String::from(new_tip_hash); //* Accede al contenido del la variable protegida por el RwLock
}

pub fn iterator(&self)->BlockchainIterator{
    BlockchainIterator::new(self.get_tip_hash(),self.db.clone()) //Se clona la referencia al Db
    
}
pub struct BlockchainIterator {
    db: Db,
    current_hash: String,
}
impl BlockchainIterator {
    fn new(tip_hash: String, db: Db) -> BlockchainIterator {
    
    }
    pub fn next(&mut self) -> Option<Block> {

    }
}
    
pub fn mine_block(&self, transactions: &[Transaction]) -> Block {
    for trasaction in transactions {
    if trasaction.verify(self) == false {
    panic!("ERROR: Invalid transaction")
    }
    }
    let best_height = self.get_best_height();
    let block = Block::new_block(self.get_tip_hash(), transactions, best_height + 1);
    let block_hash = block.get_hash();
    let blocks_tree = self.db.open_tree(BLOCKS_TREE).unwrap();
    Self::update_blocks_tree(&blocks_tree, &block);
    self.set_tip_hash(block_hash);
    block
}

fn update_blocks_tree(blocks_tree: &Tree, block: &Block) {
    let block_hash = block.get_hash();
    et _: TransactionResult<(), ()> = blocks_tree.transaction(|tx_db| {
    let _ = tx_db.insert(block_hash, block.clone());
    let _ = tx_db.insert(TIP_BLOCK_HASH_KEY, block_hash);
    Ok(())
    });
}
    
pub fn add_block(&self, block: &Block) {
    let block_tree = self.db.open_tree(BLOCKS_TREE).unwrap();
    if let Some(_) = block_tree.get(block.get_hash()).unwrap() {
    return;
    }
    let _: TransactionResult<(), ()> = block_tree.transaction(|tx_db| {
    let _ = tx_db.insert(block.get_hash(), block.serialize()).unwrap();
    let tip_block_bytes = tx_db
    .get(self.get_tip_hash())
    .unwrap()
    .expect("The tip hash is not valid");
    let tip_block = Block::deserialize(tip_block_bytes.as_ref());
    if block.get_height() > tip_block.get_height() {
    let _ = tx_db.insert(TIP_BLOCK_HASH_KEY, block.get_hash()).unwrap();
    self.set_tip_hash(block.get_hash());
    }
    Ok(())
    });
}
pub fn find_utxo(&self) -> HashMap<String, Vec<TXOutput>> {
    let mut utxo: HashMap<String, Vec<TXOutput>> = HashMap::new();
    let mut spent_txos: HashMap<String, Vec<usize>> = HashMap::new();
    let mut iterator = self.iterator();
    loop {
    let option = iterator.next();
        if option.is_none() {
        break;
    }
    let block = option.unwrap();
        'outer: for tx in block.get_transactions() {
    let txid_hex = HEXLOWER.encode(tx.get_id());
        for (idx, out) in tx.get_vout().iter().enumerate() {
            }
        }
    }
    utxo
}
pub fn find_transaction(&self, txid: &[u8]) -> Option<Transaction> {
    let mut iterator = self.iterator();
        loop {
    let option = iterator.next();
        if option.is_none() {
        break;
    }
    let block = option.unwrap();
        for transaction in block.get_transactions() {
            if txid.eq(transaction.get_id()) {
            return Some(transaction.clone());
                }
            }
        }
        None
}

pub struct Server {
    blockchain: Blockchain,
}
impl Server {
    pub fn new(blockchain: Blockchain) -> Server {
        Server { blockchain }
    }
    pub fn run(&self, addr: &str) {
        let listener = TcpListener::bind(addr).unwrap();
        if addr.eq(CENTERAL_NODE) == false {
            let best_height = self.blockchain.get_best_height();
            send_version(CENTERAL_NODE, best_height);
        }
        for stream in listener.incoming() {
            let blockchain = self.blockchain.clone();
            thread::spawn(|| match stream {
                Ok(stream) => {
                }
                Err(e) => {
                }
            });
        }
    }
}
    
fn send_get_data(addr: &str, op_type: OpType, id: &[u8]) {
    let socket_addr = addr.parse().unwrap();
    let node_addr = GLOBAL_CONFIG.get_node_addr().parse().unwrap();
    send_data(
        socket_addr,
        Package::GetData {
            addr_from: node_addr,
            op_type,
            id: id.to_vec(),
        },
    );
}

fn send_inv(addr: &str, op_type: OpType, blocks: &[Vec<u8>]) {
    let socket_addr = addr.parse().unwrap();
    let node_addr = GLOBAL_CONFIG.get_node_addr().parse().unwrap();
    send_data(
        socket_addr,
        Package::Inv {
            addr_from: node_addr,
            op_type,
            items: blocks.to_vec(),
        },
    );
}
fn send_block(addr: &str, block: &Block) {
    let socket_addr = addr.parse().unwrap();
    let node_addr = GLOBAL_CONFIG.get_node_addr().parse().unwrap();
    send_data(
        socket_addr,
        Package::Block {
            addr_from: node_addr,
            block: block.serialize(),
        },
    );
}
fn send_version(addr: &str, height: usize) {
    let socket_addr = addr.parse().unwrap();
    let node_addr = GLOBAL_CONFIG.get_node_addr().parse().unwrap();
    send_data(
        socket_addr,
        Package::Version {
            addr_from: node_addr,
            version: NODE_VERSION,
            best_height: height,
        },  
    );
}
fn send_get_blocks(addr: &str) {
    let socket_addr = addr.parse().unwrap();
    let node_addr = GLOBAL_CONFIG.get_node_addr().parse().unwrap();
    send_data(
        socket_addr,
        Package::GetBlocks {
        addr_from: node_addr,
        },
    );
}
fn serve(blockchain: Blockchain, stream: TcpStream) -> Result<(), Box<dyn Error>> {
let _ = stream.shutdown(Shutdown::Both);
    Ok(())
}
    

#[derive(Clone)]
pub struct Node {
        addr: String,
}
impl Node {
    fn new(addr: String) -> Node {
        Node { addr }
    }
    pub fn get_addr(&self) -> String {
        self.addr.clone()
    }
    pub fn parse_socket_addr(&self) -> SocketAddr {
        self.addr.parse().unwrap()
    }
}
pub struct Nodes {
    inner: RwLock<Vec<Node>>,
}
impl Nodes {
    pub fn new() -> Nodes {
    Nodes {
    inner: RwLock::new(vec![]),
    }
    }
    pub fn add_node(&self, addr: String) {
    
    }
    pub fn evict_node(&self, addr: &str) {
   
    }
    pub fn first(&self) -> Option<Node> {
   
    }
    pub fn get_nodes(&self) -> Vec<Node> {
   
    }
    pub fn len(&self) -> usize {
    
    }
    pub fn node_is_known(&self, addr: &str) -> bool {
    
    }
    }
    