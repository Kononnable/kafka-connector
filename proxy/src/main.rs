use bytes::Bytes;
use futures_util::FutureExt;
use kafka_connector_protocol::{
    api::{
        add_offsets_to_txn::AddOffsetsToTxnRequest,
        add_partitions_to_txn::AddPartitionsToTxnRequest,
        alter_client_quotas::AlterClientQuotasRequest, alter_configs::AlterConfigsRequest,
        alter_isr::AlterIsrRequest,
        alter_partition_reassignments::AlterPartitionReassignmentsRequest,
        alter_replica_log_dirs::AlterReplicaLogDirsRequest,
        alter_user_scram_credentials::AlterUserScramCredentialsRequest,
        api_versions::ApiVersionsRequest, controlled_shutdown::ControlledShutdownRequest,
        create_acls::CreateAclsRequest, create_delegation_token::CreateDelegationTokenRequest,
        create_partitions::CreatePartitionsRequest, create_topics::CreateTopicsRequest,
        delete_acls::DeleteAclsRequest, delete_groups::DeleteGroupsRequest,
        delete_records::DeleteRecordsRequest, delete_topics::DeleteTopicsRequest,
        describe_acls::DescribeAclsRequest, describe_client_quotas::DescribeClientQuotasRequest,
        describe_configs::DescribeConfigsRequest,
        describe_delegation_token::DescribeDelegationTokenRequest,
        describe_groups::DescribeGroupsRequest, describe_log_dirs::DescribeLogDirsRequest,
        describe_user_scram_credentials::DescribeUserScramCredentialsRequest,
        elect_leaders::ElectLeadersRequest, end_txn::EndTxnRequest,
        expire_delegation_token::ExpireDelegationTokenRequest, fetch::FetchRequest,
        find_coordinator::FindCoordinatorRequest, heartbeat::HeartbeatRequest,
        incremental_alter_configs::IncrementalAlterConfigsRequest,
        init_producer_id::InitProducerIdRequest, join_group::JoinGroupRequest,
        leader_and_isr::LeaderAndIsrRequest, leave_group::LeaveGroupRequest,
        list_groups::ListGroupsRequest, list_offsets::ListOffsetsRequest,
        list_partition_reassignments::ListPartitionReassignmentsRequest, metadata::MetadataRequest,
        offset_commit::OffsetCommitRequest, offset_delete::OffsetDeleteRequest,
        offset_fetch::OffsetFetchRequest, offset_for_leader_epoch::OffsetForLeaderEpochRequest,
        produce::ProduceRequest, renew_delegation_token::RenewDelegationTokenRequest,
        sasl_authenticate::SaslAuthenticateRequest, sasl_handshake::SaslHandshakeRequest,
        stop_replica::StopReplicaRequest, sync_group::SyncGroupRequest,
        txn_offset_commit::TxnOffsetCommitRequest, update_features::UpdateFeaturesRequest,
        update_metadata::UpdateMetadataRequest, write_txn_markers::WriteTxnMarkersRequest,
        ApiNumbers,
    },
    from_bytes::FromBytes,
    ApiCall,
};
use tokio::{io, net::tcp::ReadHalf};
use tokio::{
    io::AsyncReadExt,
    net::{TcpListener, TcpStream},
};
use tokio::{io::AsyncWriteExt, net::tcp::WriteHalf};

use std::env;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let listen_addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:9091".to_string());
    let server_addr = env::args()
        .nth(2)
        .unwrap_or_else(|| "127.0.0.1:9092".to_string());

    println!("Listening on: {}", listen_addr);
    println!("Proxying to: {}", server_addr);

    let listener = TcpListener::bind(listen_addr).await?;

    while let Ok((inbound, _)) = listener.accept().await {
        let transfer = transfer(inbound, server_addr.clone()).map(|r| {
            if let Err(e) = r {
                println!("Failed to transfer; error={}", e);
            }
        });

        tokio::spawn(transfer);
    }

    Ok(())
}

async fn transfer(mut inbound: TcpStream, proxy_addr: String) -> Result<(), Box<dyn Error>> {
    println!("New connection from {:#?}", inbound.local_addr());
    let mut outbound = TcpStream::connect(proxy_addr).await?;

    let (ri, wi) = inbound.split();
    let (ro, wo) = outbound.split();

    tokio::try_join!(handle_broker_input(ri, wo), handle_broker_output(ro, wi))?;

    println!("Connection closed");
    Ok(())
}

async fn handle_broker_input(mut ri: ReadHalf<'_>, mut wo: WriteHalf<'_>) -> io::Result<()> {
    loop {
        let mut size: [u8; 4] = [0, 0, 0, 0];
        let result_len = ri.read_exact(&mut size).await;
        if result_len.is_err() {
            break;
        }
        let cap = i32::from_be_bytes(size);
        wo.write_all(&cap.to_be_bytes()).await.unwrap();

        let mut buf2 = vec![0; cap as usize];
        ri.read_exact(&mut buf2).await.unwrap();
        let buf2 = Bytes::from(buf2);
        wo.write_all(&buf2).await.unwrap();

        let (api_key, api_version) = peek_key_and_version(&buf2);
        let api = ApiNumbers::from_i16(api_key);
        println!("Key: {:#?} Version: {}", api, api_version);
        println!("{}", deserialize_request(buf2, api, api_version));
    }
    wo.shutdown().await
}

fn peek_key_and_version(buf: &Bytes) -> (i16, u16) {
    let mut slice = buf.slice(0..4);
    let key: i16 = FromBytes::deserialize(&mut slice, false, 0);
    let version: i16 = FromBytes::deserialize(&mut slice, false, 0);
    (key, version as u16)
}
async fn handle_broker_output(mut ro: ReadHalf<'_>, mut wi: WriteHalf<'_>) -> io::Result<()> {
    io::copy(&mut ro, &mut wi).await?;
    wi.shutdown().await
}

fn deserialize_request(mut buf: Bytes, api: ApiNumbers, version: u16) -> String {
    match api {
        ApiNumbers::Produce => format!(
            "{:#?}",
            ProduceRequest::deserialize_request(version, &mut buf,)
        ),
        ApiNumbers::Fetch => format!(
            "{:#?}",
            FetchRequest::deserialize_request(version, &mut buf,)
        ),
        ApiNumbers::ListOffsets => {
            format!(
                "{:#?}",
                ListOffsetsRequest::deserialize_request(version, &mut buf,)
            )
        }
        ApiNumbers::Metadata => format!(
            "{:#?}",
            MetadataRequest::deserialize_request(version, &mut buf,)
        ),
        ApiNumbers::LeaderAndIsr => {
            format!(
                "{:#?}",
                LeaderAndIsrRequest::deserialize_request(version, &mut buf,)
            )
        }
        ApiNumbers::StopReplica => {
            format!(
                "{:#?}",
                StopReplicaRequest::deserialize_request(version, &mut buf,)
            )
        }
        ApiNumbers::UpdateMetadata => format!(
            "{:#?}",
            UpdateMetadataRequest::deserialize_request(version, &mut buf,)
        ),
        ApiNumbers::ControlledShutdown => format!(
            "{:#?}",
            ControlledShutdownRequest::deserialize_request(version, &mut buf,)
        ),
        ApiNumbers::OffsetCommit => {
            format!(
                "{:#?}",
                OffsetCommitRequest::deserialize_request(version, &mut buf,)
            )
        }
        ApiNumbers::OffsetFetch => {
            format!(
                "{:#?}",
                OffsetFetchRequest::deserialize_request(version, &mut buf,)
            )
        }
        ApiNumbers::FindCoordinator => format!(
            "{:#?}",
            FindCoordinatorRequest::deserialize_request(version, &mut buf,)
        ),
        ApiNumbers::JoinGroup => format!(
            "{:#?}",
            JoinGroupRequest::deserialize_request(version, &mut buf,)
        ),
        ApiNumbers::Heartbeat => format!(
            "{:#?}",
            HeartbeatRequest::deserialize_request(version, &mut buf,)
        ),
        ApiNumbers::LeaveGroup => {
            format!(
                "{:#?}",
                LeaveGroupRequest::deserialize_request(version, &mut buf,)
            )
        }
        ApiNumbers::SyncGroup => format!(
            "{:#?}",
            SyncGroupRequest::deserialize_request(version, &mut buf,)
        ),
        ApiNumbers::DescribeGroups => format!(
            "{:#?}",
            DescribeGroupsRequest::deserialize_request(version, &mut buf,)
        ),
        ApiNumbers::ListGroups => {
            format!(
                "{:#?}",
                ListGroupsRequest::deserialize_request(version, &mut buf,)
            )
        }
        ApiNumbers::SaslHandshake => {
            format!(
                "{:#?}",
                SaslHandshakeRequest::deserialize_request(version, &mut buf,)
            )
        }
        ApiNumbers::ApiVersions => {
            format!(
                "{:#?}",
                ApiVersionsRequest::deserialize_request(version, &mut buf,)
            )
        }
        ApiNumbers::CreateTopics => {
            format!(
                "{:#?}",
                CreateTopicsRequest::deserialize_request(version, &mut buf,)
            )
        }
        ApiNumbers::DeleteTopics => {
            format!(
                "{:#?}",
                DeleteTopicsRequest::deserialize_request(version, &mut buf,)
            )
        }
        ApiNumbers::DeleteRecords => {
            format!(
                "{:#?}",
                DeleteRecordsRequest::deserialize_request(version, &mut buf,)
            )
        }
        ApiNumbers::InitProducerId => format!(
            "{:#?}",
            InitProducerIdRequest::deserialize_request(version, &mut buf,)
        ),
        ApiNumbers::OffsetForLeaderEpoch => {
            format!(
                "{:#?}",
                OffsetForLeaderEpochRequest::deserialize_request(version, &mut buf,)
            )
        }
        ApiNumbers::AddPartitionsToTxn => format!(
            "{:#?}",
            AddPartitionsToTxnRequest::deserialize_request(version, &mut buf,)
        ),
        ApiNumbers::AddOffsetsToTxn => format!(
            "{:#?}",
            AddOffsetsToTxnRequest::deserialize_request(version, &mut buf,)
        ),
        ApiNumbers::EndTxn => format!(
            "{:#?}",
            EndTxnRequest::deserialize_request(version, &mut buf,)
        ),
        ApiNumbers::WriteTxnMarkers => format!(
            "{:#?}",
            WriteTxnMarkersRequest::deserialize_request(version, &mut buf,)
        ),
        ApiNumbers::TxnOffsetCommit => format!(
            "{:#?}",
            TxnOffsetCommitRequest::deserialize_request(version, &mut buf,)
        ),
        ApiNumbers::DescribeAcls => {
            format!(
                "{:#?}",
                DescribeAclsRequest::deserialize_request(version, &mut buf,)
            )
        }
        ApiNumbers::CreateAcls => {
            format!(
                "{:#?}",
                CreateAclsRequest::deserialize_request(version, &mut buf,)
            )
        }
        ApiNumbers::DeleteAcls => {
            format!(
                "{:#?}",
                DeleteAclsRequest::deserialize_request(version, &mut buf,)
            )
        }
        ApiNumbers::DescribeConfigs => format!(
            "{:#?}",
            DescribeConfigsRequest::deserialize_request(version, &mut buf,)
        ),
        ApiNumbers::AlterConfigs => {
            format!(
                "{:#?}",
                AlterConfigsRequest::deserialize_request(version, &mut buf,)
            )
        }
        ApiNumbers::AlterReplicaLogDirs => {
            format!(
                "{:#?}",
                AlterReplicaLogDirsRequest::deserialize_request(version, &mut buf,)
            )
        }
        ApiNumbers::DescribeLogDirs => format!(
            "{:#?}",
            DescribeLogDirsRequest::deserialize_request(version, &mut buf,)
        ),
        ApiNumbers::SaslAuthenticate => format!(
            "{:#?}",
            SaslAuthenticateRequest::deserialize_request(version, &mut buf,)
        ),
        ApiNumbers::CreatePartitions => format!(
            "{:#?}",
            CreatePartitionsRequest::deserialize_request(version, &mut buf,)
        ),
        ApiNumbers::CreateDelegationToken => {
            format!(
                "{:#?}",
                CreateDelegationTokenRequest::deserialize_request(version, &mut buf,)
            )
        }
        ApiNumbers::RenewDelegationToken => {
            format!(
                "{:#?}",
                RenewDelegationTokenRequest::deserialize_request(version, &mut buf,)
            )
        }
        ApiNumbers::ExpireDelegationToken => {
            format!(
                "{:#?}",
                ExpireDelegationTokenRequest::deserialize_request(version, &mut buf,)
            )
        }
        ApiNumbers::DescribeDelegationToken => {
            format!(
                "{:#?}",
                DescribeDelegationTokenRequest::deserialize_request(version, &mut buf,)
            )
        }
        ApiNumbers::DeleteGroups => {
            format!(
                "{:#?}",
                DeleteGroupsRequest::deserialize_request(version, &mut buf,)
            )
        }
        ApiNumbers::ElectLeaders => {
            format!(
                "{:#?}",
                ElectLeadersRequest::deserialize_request(version, &mut buf,)
            )
        }
        ApiNumbers::IncrementalAlterConfigs => {
            format!(
                "{:#?}",
                IncrementalAlterConfigsRequest::deserialize_request(version, &mut buf,)
            )
        }
        ApiNumbers::AlterPartitionReassignments => {
            format!(
                "{:#?}",
                AlterPartitionReassignmentsRequest::deserialize_request(version, &mut buf,)
            )
        }
        ApiNumbers::ListPartitionReassignments => {
            format!(
                "{:#?}",
                ListPartitionReassignmentsRequest::deserialize_request(version, &mut buf,)
            )
        }
        ApiNumbers::OffsetDelete => format!(
            "{:#?}",
            OffsetDeleteRequest::deserialize_request(version, &mut buf,)
        ),
        ApiNumbers::DescribeClientQuotas => {
            format!(
                "{:#?}",
                DescribeClientQuotasRequest::deserialize_request(version, &mut buf,)
            )
        }
        ApiNumbers::AlterClientQuotas => format!(
            "{:#?}",
            AlterClientQuotasRequest::deserialize_request(version, &mut buf,)
        ),
        ApiNumbers::DescribeUserScramCredentials => {
            format!(
                "{:#?}",
                DescribeUserScramCredentialsRequest::deserialize_request(version, &mut buf,)
            )
        }
        ApiNumbers::AlterUserScramCredentials => {
            format!(
                "{:#?}",
                AlterUserScramCredentialsRequest::deserialize_request(version, &mut buf,)
            )
        }
        ApiNumbers::AlterIsr => format!(
            "{:#?}",
            AlterIsrRequest::deserialize_request(version, &mut buf,)
        ),
        ApiNumbers::UpdateFeatures => format!(
            "{:#?}",
            UpdateFeaturesRequest::deserialize_request(version, &mut buf,)
        ),
    }
}
