//! Tests for the module.
use super::*;
//use sr_primitives::traits::SignedExtension;
//use support::{
//	assert_err, assert_noop, assert_ok,
//	traits::{Currency, ExistenceRequirement::AllowDeath, LockableCurrency, ReservableCurrency},
//};
//use system::RawOrigin;

use mock::{EthRelay, ExtBuilder, System};
//use mock::{info_from_weight, EthRelay, ExtBuilder, Runtime, System, CALL};

//use rstd::prelude::*;
use sr_eth_primitives::{
	receipt::{LogEntry, TransactionOutcome},
	Address, Bloom, H64, U128,
};

use hex_literal::hex;
use rustc_hex::FromHex;
use sha3::{Digest, Keccak256, Keccak512};
use std::str::FromStr;

#[test]
fn verify_receipt_proof() {
	ExtBuilder::default()
		.existential_deposit(256 * 1)
		.monied(true)
		.build()
		.execute_with(|| {
			System::inc_account_nonce(&2);

			let log_entries = vec![LogEntry {
				address: Address::from_str("ad52e0f67b6f44cd5b9a6f4fbc7c0f78f37e094b").unwrap(),
				topics: vec![
					H256::from(hex!("6775ce244ff81f0a82f87d6fd2cf885affb38416e3a04355f713c6f008dd126a")),
					H256::from(hex!("0000000000000000000000000000000000000000000000000000000000000006")),
					H256::from(hex!("0000000000000000000000000000000000000000000000000000000000000000")),
				],
				data: "00000000000000000000000074241db5f3ebaeecf9506e4ae9881860933416048eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48000000000000000000000000000000000000000000000000002386f26fc10000".from_hex().unwrap(),
			}];

			let receipt = Receipt::new(
				TransactionOutcome::StatusCode(1),
//				TransactionOutcome::StateRoot(H256::from(hex!("a21cdf375ebef58f606c298d6211f4edee58f2dd6430edbdd0ed3cd886a16863"))),
				U256::from(U128::from(1123401)),
				log_entries
			);


			let proof_record = ActionRecord {
				index: 25,
				proof: "f904c4f904c1b8b3f8b1a0c75e4fe93609c5f088e180e294577ba0f991fcad25e6163523adba4bfc65cfa8a008d8d33daaf581590c70f28317e5a48c33786ee092d7d9a9b4faae64fd05339ba0562b932c3332c149c7449d68be351f41c947c5f4b6d336906970f361dc905c67a0da77a1e9b271dcaaf156d5528be7e6c586930feab5d0e644208c0b8e54eed21780808080a0e58215be848c1293dd381210359d84485553000a82b67410406d183b42adbbdd8080808080808080b90214f90211a08fd1196d29f53e148b7cd38b1143b132d8f9bd4a9c5a2ad51244de514b5b5f19a0a6d91f439a4b87ec5861732d4900baa7df91c8b2f0f02eb9c0e640269adcae3da00cbe602772266b03258721442dd7327eb996fb2eef54b4fbe77c9b57053dd3f5a0e412c05734ae17fa87154402c9737bfd800f44aa3df0ef32fe56092214868b87a0a60ac628f42d20e1dee3d479c192b74ceacbb7d571a93750132c536328b031a6a03518806a81c734f33fe971a22721c12f2f3cc60d7f9b3bc89403d7cfdb5d0895a0d130ed44f0def9f86a53d3e3720615cec6f6f0aedecd4fc0cb2649c766ca1a17a0d421bfc8d9f46e123e432b8582c49629a969547a8ef40b231659b8385c7c1b81a09a62e4ae73121a710ba5353172874f248df38f39ceaef351522c4a9b1cffb1c3a09f4604347f9ba2c30703cce323c9f9705e0edecf5c1061e634a792de9a854e00a015421788d874414ca073e71d99c5fab4acd350b46551a48aa29891d322651071a0a1f624aded3a70996b4117dc609e5fbdd1bbdc819935be31a395904a1f85982aa0a69eb11de6f2d70d0ab095da5ba88f38cd9a60569839ecf35103360603d9aa2da02564a45d7661a773b13f984a47c63017fcea8599b39f42df99d1132d9cf2c159a0ff8b9f7b23ffe706af9188e74da6ad7ead36ba7d75c47ef915541689cc025194a094974e354978838330aeefefe0e29fa2e86cab1f4503b1b895f889514f48aa0e80b901f2f901ef20b901ebf901e80183112449b9010000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000000400000000000000000000000000000000000000000000000000000000000000000000000000000820000000000000020000000000000000000800000000000000000000000000000000000000000400000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000080000000000000000000000000000000000000000000000000000000000200000000020000000000000000000000000000080000000000000800000000000000000000000f8def8dc94ad52e0f67b6f44cd5b9a6f4fbc7c0f78f37e094bf863a06775ce244ff81f0a82f87d6fd2cf885affb38416e3a04355f713c6f008dd126aa00000000000000000000000000000000000000000000000000000000000000006a00000000000000000000000000000000000000000000000000000000000000000b86000000000000000000000000074241db5f3ebaeecf9506e4ae9881860933416048eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48000000000000000000000000000000000000000000000000002386f26fc10000".from_hex().unwrap(),
				header_hash: H256::from(hex!("f1a5bc27877e219b859b0bb1f2f440134553019f9bb5a2eca7a4703263e736c9"))
			};

//			let proof: Proof = rlp::decode(&proof_record.proof).unwrap();

			let mixh = H256::from(hex!("5a85e328a8bb041a386ffb25db029b7f0df4665a8a55b331b30a576761404fa6"));
			let nonce = H64::from(hex!("650ea83006bb108d"));

			let header = EthHeader {
				parent_hash: H256::from(hex!("91553997d11a1d978f2ea363f230f5f525aee914a726d01e1deb4ea51de315cd")),
				timestamp: 1573560715,
				number: 6760579,
				author: Address::from(hex!("d7a15baeb7ea05c9660cbe03fb7999c2c2e57625")),
				transactions_root: H256::from(hex!("c2b9e612bdac9d73d53ab38cafa959e5703dc078a9d5b184c65ee38bc471b5bf")),
				uncles_hash: H256::from(hex!("1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347")),
				extra_data: "41746c616e7469632043727970746f".from_hex().unwrap(),
				state_root: H256::from(hex!("a21cdf375ebef58f606c298d6211f4edee58f2dd6430edbdd0ed3cd886a16863")),
				receipts_root: H256::from(hex!("4c573edd96da310fefc3ced2d70831173e4684728c963330d990cf360aed8550")),
				log_bloom: Bloom::from_str("040000411080018200400100100020100808080020130000004000000a80040000001000000400004010800004811000000000800604002004000000002300820008181000000a820142010c0000010418030040080010080010280018200408000020800208120100000000001828000000000200000800000080511508c0008004100482000800040080000411409000000d20400000056000000802400006420002801000108140202100000804109008000150800140000020290028404000040102800000002000020000811004020080008000100411300100422420060210100100110124080000800084022021000200808005500000000000012000").unwrap(),
				gas_used: 0x220d13.into(),
				gas_limit: 0x7a121d.into(),
				difficulty: 0x269921540_u64.into(),
				seal: vec![rlp::encode(&mixh), rlp::encode(&nonce)],
				hash: Some(H256::from(hex!("f1a5bc27877e219b859b0bb1f2f440134553019f9bb5a2eca7a4703263e736c9"))),
			};

			EthRelay::init_genesis_header(&header, 0x624c22d93f8e59_u64);

			assert_eq!(EthRelay::verify_receipt(&proof_record), Some(receipt));
		});
}

#[test]
fn relay_header() {
	ExtBuilder::default().monied(true).build().execute_with(|| {
		// 6760579
//		let mixh1 = H256::from(hex!("5a85e328a8bb041a386ffb25db029b7f0df4665a8a55b331b30a576761404fa6"));
//		let nonce1 = H64::from(hex!("650ea83006bb108d"));
//
//		let header1 = EthHeader {
//			parent_hash: H256::from(hex!("91553997d11a1d978f2ea363f230f5f525aee914a726d01e1deb4ea51de315cd")),
//			timestamp: 1573560715,
//			number: 6760579,
//			author: Address::from(hex!("d7a15baeb7ea05c9660cbe03fb7999c2c2e57625")),
//			transactions_root: H256::from(hex!("c2b9e612bdac9d73d53ab38cafa959e5703dc078a9d5b184c65ee38bc471b5bf")),
//			uncles_hash: H256::from(hex!("1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347")),
//			extra_data: "41746c616e7469632043727970746f".from_hex().unwrap(),
//			state_root: H256::from(hex!("a21cdf375ebef58f606c298d6211f4edee58f2dd6430edbdd0ed3cd886a16863")),
//			receipts_root: H256::from(hex!("4c573edd96da310fefc3ced2d70831173e4684728c963330d990cf360aed8550")),
//			log_bloom: Bloom::from_str("040000411080018200400100100020100808080020130000004000000a80040000001000000400004010800004811000000000800604002004000000002300820008181000000a820142010c0000010418030040080010080010280018200408000020800208120100000000001828000000000200000800000080511508c0008004100482000800040080000411409000000d20400000056000000802400006420002801000108140202100000804109008000150800140000020290028404000040102800000002000020000811004020080008000100411300100422420060210100100110124080000800084022021000200808005500000000000012000").unwrap(),
//			gas_used: 0x220d13.into(),
//			gas_limit: 0x7a121d.into(),
//			difficulty: 0x269921540_u64.into(),
//			seal: vec![rlp::encode(&mixh1), rlp::encode(&nonce1)],
//			hash: Some(H256::from(hex!("f1a5bc27877e219b859b0bb1f2f440134553019f9bb5a2eca7a4703263e736c9"))),
//		};

		let mixh1 = H256::from(hex!("c4b28f4b671b2e675634f596840d3115ce3df0ab38b6608a69371da16a3455aa"));
		let nonce1 = H64::from(hex!("7afbefa403b138fa"));
		// #6890091
		// https://api-ropsten.etherscan.io/api?module=proxy&action=eth_getBlockByNumber&tag=0x69226b&boolean=true&apikey=YourApiKeyToken
		// https://jsoneditoronline.org/
		let header1 = EthHeader {
			parent_hash: H256::from(hex!("8a18726cacb45b078bfe6491510cfa2dd578a70be2a217f416253cf3e94adbd2")),
			timestamp: 0x5de5246c,
			number: 0x69226b,
			author: Address::from(hex!("4ccfb3039b78d3938588157564c9ad559bafab94")),
			transactions_root: H256::from(hex!("e3ab46e9eeb65fea6b0b1ffd07587f3ee7741b66f16a0b63a3b0c01900387833")),
			uncles_hash: H256::from(hex!("1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347")),
			extra_data: "d983010906846765746889676f312e31312e3133856c696e7578".from_hex().unwrap(),
			state_root: H256::from(hex!("de1df18f7da776a86119d17373d252d3591b5a4270e14113701d27c852d25313")),
			receipts_root: H256::from(hex!("9c9eb20b6f9176864630f84aa11f33969a355efa85b2eb1e386a5b1ea3599089")),
			log_bloom: Bloom::from_str("0420000400000018000400400402044000088100000088000000010000040800202000002000a0000000000200004000800100000200000000000020003400000000000004002000000000080102004400000000010400008001000000000020000000009200100000000000004408040100000010000010022002130002000600048200000000000000004000002410000008000000000008021800100000000704010008080000200081000000004002000000009010c000010082000040400104020200000000040180000000000a803000000000002212000000000061000010000001010000400020000000002000020008008100040000005200000000").unwrap(),
			gas_used: 0x769975.into(),
			gas_limit: 0x7a1200.into(),
			difficulty: 0xf4009f4b_u64.into(),
			seal: vec![rlp::encode(&mixh1), rlp::encode(&nonce1)],
			hash: Some(H256::from(hex!("1dafbf6a9825241ea5dfa7c3a54781c0784428f2ef3b588748521f83209d3caa"))),
		};

		type DAG = LightDAG<EthereumPatch>;

//		let light_dag1 = DAG::new(header1.number().into());
//		let partial_header_hash1 = header1.bare_hash();
//		let mixhash1 = light_dag1
//			.hashimoto(partial_header_hash1, nonce1)
//			.0;
//		assert_eq!(
//			mixhash1,
//			mixh1
//		);


		// 6760580
		let mixh2 = H256::from(hex!("e06f0c107dcc91e9e82de0b42d0e22d5c2cfae5209422fda88cff4f810f4bffb"));
		let nonce2 = H64::from(hex!("9348d06003756cff"));

		let header2 = EthHeader {
			parent_hash: H256::from(hex!("f1a5bc27877e219b859b0bb1f2f440134553019f9bb5a2eca7a4703263e736c9")),
			timestamp: 0x5dcaa1a3,
			number: 6760580,
			author: Address::from(hex!("4ccfb3039b78d3938588157564c9ad559bafab94")),
			transactions_root: H256::from(hex!("bd4f8075fcdf01d3be2b8ae4a0a7195107429f34361e278e8760cc0f08e35d7a")),
			uncles_hash: H256::from(hex!("1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347")),
			extra_data: "d983010906846765746889676f312e31312e3133856c696e7578".from_hex().unwrap(),
			state_root: H256::from(hex!("694af9f7dc9866ec99dd83ef846778552cb60659e9cbd6e77e800816da83c3c9")),
			receipts_root: H256::from(hex!("729394331d204a175e4c1938ae19cc905107d8fd5562ee5283c323cde6b82e23")),
			log_bloom: Bloom::from_str("0400000000000100001000100000040000000100000000000000000002040080002004000000000200000000000210000080000002000080000000040014000000000000040020000000000800020040080110000004008800000000000000000100000002000000000000000000080040000000000004000010801101000000000000000000000000000000020060000000001000020000200002000000100000000000000000001000010000000000000001000080000000011000002040401000001280000000000021000800000800000000000010000000000040006000000400200000000000000000000000000000000000c000100000400000800100").unwrap(),
			gas_used: 0x17231e.into(),
			gas_limit: 0x7a1200.into(),
			difficulty: 0x2694562fe_u64.into(),
			seal: vec![rlp::encode(&mixh2), rlp::encode(&nonce2)],
			hash: Some(H256::from(hex!("12734378d3e4ad7050f7baf629d6eda161e911865d77c10e44c1f7e8e31fd7a7"))),
		};


		EthRelay::init_genesis_header(&header1, 0x624c22d93f8e59_u64);

		let light_dag2 = DAG::new(header2.number().into());
		let partial_header_hash2 = header2.bare_hash();

//		println!("partial_header_hash2: {:?}", rlp::encode(&mixh2));

		let mixhash2 = light_dag2
			.hashimoto(partial_header_hash2, nonce2)
			.0;
		assert_eq!(
			mixhash2,
			mixh2
		);

		EthRelay::verify_header(&header2).expect("Verify Failed.");

		EthRelay::store_header(&header2).expect("Store Failed.");


		// 6760581
		let mixh3 = H256::from(hex!("019b6a52120a8769d34fe6348bdfa400ab4886576287f5ef11d9105875280c7e"));
		let nonce3 = H64::from(hex!("f43d6b58a23b7065"));

		let header3 = EthHeader {
			parent_hash: H256::from(hex!("12734378d3e4ad7050f7baf629d6eda161e911865d77c10e44c1f7e8e31fd7a7")),
			timestamp: 0x5dcaa1ae,
			number: 6760581,
			author: Address::from(hex!("d7a15baeb7ea05c9660cbe03fb7999c2c2e57625")),
			transactions_root: H256::from(hex!("aaccb1d4b2dc847eefa50681d3096522a41f7c27031ead7a0ad51b50632218dc")),
			uncles_hash: H256::from(hex!("1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347")),
			extra_data: "41746c616e7469632043727970746f".from_hex().unwrap(),
			state_root: H256::from(hex!("8106951604cc1305eedb3b7df1c2cf9c2d0ba9e792f645386d3a2fdffd2e9d96")),
			receipts_root: H256::from(hex!("e39a6c035914d6544db6d3653101740625e7608c747ea87b9784261e5d94a7ea")),
			log_bloom: Bloom::from_str("00000000000001000000000000000000000000000000000000000000000000000000000000000020000000000000000000200020400000000000000000000000000000000000000000000008000000000000080000000000000000000200000000000000000000000000000000008100000000000000000000000010010000000000020000000000000000000000040000000010040000002000204000000000000000000000000000000100000000000000000000000050002000000000000000800002800000000400000000000000000040000000100000000200000000080000000400002000000000000000000000002000000000000000000002020000").unwrap(),
			gas_used: 0x3ea15.into(),
			gas_limit: 0x7a121d.into(),
			difficulty: 0x26945e2fe_u64.into(),
			seal: vec![rlp::encode(&mixh3), rlp::encode(&nonce3)],
			hash: Some(H256::from(hex!("c86b090d12fa61c34f075530618e40a89654d8d85ac6aaa26149fb56b596a15a"))),
		};

//		let light_dag3 = DAG::new(header3.number().into());
//		let partial_header_hash3 = header3.bare_hash();
//		let mixhash3 = light_dag3
//			.hashimoto(partial_header_hash3, nonce3)
//			.0;
//		assert_eq!(
//			mixhash3,
//			mixh3
//		);

		EthRelay::verify_header(&header3).expect("Verify Failed.");

		EthRelay::store_header(&header3).expect("Store Failed.");
	});
}

#[test]
fn test_mainet_header_bare_hash() {
	// 8996777
	let mixh2 = H256::from(hex!("543bc0769f7d5df30e7633f4a01552c2cee7baace8a6da37fddaa19e49e81209"));
	let nonce2 = H64::from(hex!("a5d3d0ccc8bb8a29"));

	let header2 = EthHeader {
		parent_hash: H256::from(hex!("0b2d720b8d3b6601e4207ef926b0c228735aa1d58301a23d58f9cb51ac2288d8")),
		timestamp: 0x5ddb67a0,
		number: 0x8947a9,
		author: Address::from(hex!("4c549990a7ef3fea8784406c1eecc98bf4211fa5")),
		transactions_root: H256::from(hex!("07d44fadb4aca78c81698710211c5399c1408bb3f0aa3a687d091d230fcaddc6")),
		uncles_hash: H256::from(hex!("1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347")),
		extra_data: "5050594520686976656f6e2d6574682d6672".from_hex().unwrap(),
		state_root: H256::from(hex!("4ba0fb3e6f4c1af32a799df667d304bcdb7f8154e6f86831f92f5a354c2baf70")),
		receipts_root: H256::from(hex!("5968afe6026e673df3b9745d925a5648282d2195a46c22771fec48210daf8e23")),
		log_bloom: Bloom::from_str("0c7b091bc8ec02401ad12491004e3014e8806390031950181c118580ac61c9a00409022c418162002710a991108a11ca5383d4921d1da46346edc3eb8068481118b005c0b20700414c13916c54011a0922904aa6e255406a33494c84a1426410541819070e04852042410b30030d4c88a5103082284c7d9bd42090322ae883e004224e18db4d858a0805d043e44a855400945311cb253001412002ea041a08e30394fc601440310920af2192dc4194a03302191cf2290ac0c12000815324eb96a08000aad914034c1c8eb0cb39422e272808b7a4911989c306381502868820b4b95076fc004b14dd48a0411024218051204d902b80d004c36510400ccb123084").unwrap(),
		gas_used: 0x986d77.into(),
		gas_limit: 0x989631.into(),
		difficulty: 0x92ac28cbc4930_u64.into(),
		seal: vec![rlp::encode(&mixh2), rlp::encode(&nonce2)],
		hash: None,
	};

	let partial_header_hash2 = header2.bare_hash();

	assert_eq!(
		header2.hash(),
		H256::from(hex!("b80bf91d6f459227a9c617c5d9823ff0b07f1098ea16788676f0b804ecd42f3b"))
	);

	//	println!("partial_header_hash2: {:?}", partial_header_hash2);

	assert_eq!(
		//		H256::from_slice(Keccak256::digest(&rlp::encode(&header2).to_vec()).as_slice()),
		partial_header_hash2,
		H256::from(hex!("9cb3d16b788bfc7f2569db2d1fedb5b1e9633acfe84a4eca44a9fa50979a9887"))
	);
}
