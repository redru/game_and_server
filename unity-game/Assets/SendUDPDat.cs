using System;
using System.Net;
using System.Net.Sockets;
using System.Text;
using UnityEngine;

public class SendUDPDat : MonoBehaviour
{
    private static readonly byte[] HandshakeCommand = { 48, 48 }; // "00"
    private static readonly byte[] SendTimeCommand = { 48, 49 }; // "01"
    
    private string serverIP = "192.168.1.75"; // Server IP (can be localhost)
    private int sendPort = 34254; // Port to send data to the server
    private int listenPort = 34255; // Port to listen for incoming data

    private UdpClient udpClientSender;
    private UdpClient udpClientListener;
    private IPEndPoint serverEndPoint;
    private float timeSinceStart;
    
    private byte[] _guid;
    private byte[] _dataToSend;

    void Start()
    {
        // Initialize the UDP client for sending
        udpClientSender = new UdpClient();

        // Create the endpoint for the server
        serverEndPoint = new IPEndPoint(IPAddress.Parse(serverIP), sendPort);

        // Initialize the UDP client for listening
        udpClientListener = new UdpClient(listenPort);

        // Start listening for incoming UDP messages asynchronously
        udpClientListener.BeginReceive(OnReceive, null);
        
        _dataToSend = new byte[1024];

        _guid = Encoding.UTF8.GetBytes(Guid.NewGuid().ToString());

        SendHandshakeToServer();
    }

    void Update()
    {
        // Update the time since the game started
        timeSinceStart = Time.time;

        // Send the time to the server
        SendTimeToServer();
    }

    void SendHandshakeToServer()
    {
        try
        {
            var portBytes = Encoding.UTF8.GetBytes(listenPort.ToString());
            var currentOffset = 0;
            
            Buffer.BlockCopy(HandshakeCommand, 0, _dataToSend, currentOffset, HandshakeCommand.Length);
            currentOffset += HandshakeCommand.Length;
            Buffer.BlockCopy(_guid, 0, _dataToSend, currentOffset, _guid.Length);
            currentOffset += _guid.Length;
            Buffer.BlockCopy(portBytes, 0, _dataToSend, currentOffset, portBytes.Length);

            // Send the message to the server
            udpClientSender.Send(_dataToSend, _dataToSend.Length, serverEndPoint);
        }
        catch (Exception e)
        {
            Debug.LogError("Error sending data: " + e.Message);
        }
    }

    void SendTimeToServer()
    {
        try
        {
            var dataOffset = SendTimeCommand.Length + _guid.Length;
            // Clear data
            Array.Clear(_dataToSend, dataOffset, _dataToSend.Length - dataOffset);
            
            // Set proper command
            Buffer.BlockCopy(SendTimeCommand, 0, _dataToSend, 0, SendTimeCommand.Length);
            
            // Get time and send
            var timeBytes = Encoding.UTF8.GetBytes(timeSinceStart.ToString());
            Buffer.BlockCopy(timeBytes, 0, _dataToSend, dataOffset, timeBytes.Length);

            // Send the message to the server
            udpClientSender.Send(_dataToSend, _dataToSend.Length, serverEndPoint);
        }
        catch (Exception e)
        {
            Debug.LogError("Error sending data: " + e.Message);
        }
    }

    // Callback for when data is received
    void OnReceive(IAsyncResult result)
    {
        try
        {
            IPEndPoint remoteEndPoint = new IPEndPoint(IPAddress.Any, listenPort);

            // Get the received data
            byte[] receivedData = udpClientListener.EndReceive(result, ref remoteEndPoint);

            // Convert the data to string
            string receivedMessage = Encoding.UTF8.GetString(receivedData);
            Debug.Log("Received from server: " + receivedMessage);

            // Continue listening for incoming data
            udpClientListener.BeginReceive(OnReceive, null);
        }
        catch (Exception e)
        {
            Debug.LogError("Error receiving data: " + e.Message);
        }
    }

    void OnApplicationQuit()
    {
        // Close the UDP clients when the application quits
        udpClientSender.Close();
        udpClientListener.Close();
    }
}