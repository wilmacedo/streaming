import com.fasterxml.jackson.databind.SerializationFeature
import com.fasterxml.jackson.dataformat.xml.XmlMapper
import com.squareup.moshi.Json
import com.squareup.moshi.JsonClass
import com.squareup.moshi.KotlinJsonAdapterFactory
import com.squareup.moshi.Moshi
import com.sun.net.httpserver.HttpExchange
import com.sun.net.httpserver.HttpHandler
import com.sun.net.httpserver.HttpServer.create
import java.io.File
import java.io.OutputStream
import java.net.InetSocketAddress

@JsonClass(generateAdapter = true)
data class SoapData(
    @Json(name = "users") val users: List<User> = emptyList(),
    @Json(name = "musics") val musics: List<Music> = emptyList(),
    @Json(name = "playlists") val playlists: List<Playlist> = emptyList()
)

@JsonClass(generateAdapter = true)
data class User(val id: String, val name: String, val age: Int)

@JsonClass(generateAdapter = true)
data class Music(val id: String, val name: String, val author: String, val playlistId: String)

@JsonClass(generateAdapter = true)
data class Playlist(val id: String, val name: String, val userId: String)

fun main() {
    val server = create(InetSocketAddress(8080), 0)
    server.createContext("/users", UsersHandler())
    server.createContext("/musics", MusicsHandler())
    server.createContext("/userPlaylists", UserPlaylistsHandler())
    server.createContext("/playlistMusics", PlaylistMusicsHandler())
    server.createContext("/musicPlaylists", MusicPlaylistsHandler())
    server.executor = null // creates a default executor
    server.start()
    println("Server started on port 8080")
}

abstract class BaseHandler : HttpHandler {
    abstract fun handleRequest(exchange: HttpExchange)

    override fun handle(exchange: HttpExchange) {
        println("Handling request...")
        when (exchange.requestMethod) {
            "GET" -> handleRequest(exchange)
            else -> {
                exchange.sendResponseHeaders(405, -1)
                exchange.close()
            }
        }
    }
}

class UsersHandler : BaseHandler() {
    override fun handleRequest(exchange: HttpExchange) {
        try {
            val soapData = readSoapData()

            // Extract users from soapData
            val users = soapData.users

            val responseBody = convertToXml(users)

            exchange.sendResponseHeaders(200, responseBody.length.toLong())
            val os: OutputStream = exchange.responseBody
            os.write(responseBody.toByteArray())
            os.close()
            println("Response sent successfully.")
        } catch (e: Exception) {
            println("Error handling request: ${e.message}")
            exchange.sendResponseHeaders(500, -1)
            exchange.close()
        }
    }
}

class MusicsHandler : BaseHandler() {
    override fun handleRequest(exchange: HttpExchange) {
        try {
            val soapData = readSoapData()

            // Extract musics from soapData
            val musics = soapData.musics

            val responseBody = convertToXml(musics)

            exchange.sendResponseHeaders(200, responseBody.length.toLong())
            val os: OutputStream = exchange.responseBody
            os.write(responseBody.toByteArray())
            os.close()
            println("Response sent successfully.")
        } catch (e: Exception) {
            println("Error handling request: ${e.message}")
            exchange.sendResponseHeaders(500, -1)
            exchange.close()
        }
    }
}

class UserPlaylistsHandler : BaseHandler() {
    override fun handleRequest(exchange: HttpExchange) {
        try {
            val soapData = readSoapData()

            // Extract user ID from query parameters
            val userId = exchange.requestURI.query?.split("=")?.get(1)

            // Filter playlists for the specified user
            val userPlaylists = soapData.playlists.filter { it.userId == userId }

            val responseBody = convertToXml(userPlaylists)

            exchange.sendResponseHeaders(200, responseBody.length.toLong())
            val os: OutputStream = exchange.responseBody
            os.write(responseBody.toByteArray())
            os.close()
            println("Response sent successfully.")
        } catch (e: Exception) {
            println("Error handling request: ${e.message}")
            exchange.sendResponseHeaders(500, -1)
            exchange.close()
        }
    }
}

class PlaylistMusicsHandler : BaseHandler() {
    override fun handleRequest(exchange: HttpExchange) {
        try {
            val soapData = readSoapData()

            // Extract playlist ID from query parameters
            val playlistId = exchange.requestURI.query?.split("=")?.get(1)

            // Filter musics for the specified playlist
            val playlistMusics = soapData.musics.filter { it.playlistId == playlistId }

            val responseBody = convertToXml(playlistMusics)

            exchange.sendResponseHeaders(200, responseBody.length.toLong())
            val os: OutputStream = exchange.responseBody
            os.write(responseBody.toByteArray())
            os.close()
            println("Response sent successfully.")
        } catch (e: Exception) {
            println("Error handling request: ${e.message}")
            exchange.sendResponseHeaders(500, -1)
            exchange.close()
        }
    }
}

class MusicPlaylistsHandler : BaseHandler() {
    override fun handleRequest(exchange: HttpExchange) {
        try {
            val soapData = readSoapData()

            val musicId = exchange.requestURI.query?.split("=")?.get(1)
            val music = soapData.musics.find { it.id == musicId }
            val musicPlaylists = soapData.playlists.filter { it.id == music?.playlistId }

            val responseBody = convertToXml(musicPlaylists)

            exchange.sendResponseHeaders(200, responseBody.length.toLong())
            val os: OutputStream = exchange.responseBody
            os.write(responseBody.toByteArray())
            os.close()
            println("Response sent successfully.")
        } catch (e: Exception) {
            println("Error handling request: ${e.message}")
            exchange.sendResponseHeaders(500, -1)
            exchange.close()
        }
    }
}

private fun readSoapData(): SoapData {
    println("Reading SOAP data from JSON file...")
    val fileContent = File("../database.json").readText()

    val moshi = Moshi.Builder()
        .addLast(KotlinJsonAdapterFactory())
        .build()

    val adapter = moshi.adapter(SoapData::class.java)
    return adapter.fromJson(fileContent) ?: throw RuntimeException("Failed to parse JSON")
}

private fun convertToXml(data: Any): String {
    println("Converting data to XML...")
    val xmlMapper = XmlMapper()
    xmlMapper.enable(SerializationFeature.INDENT_OUTPUT)
    return xmlMapper.writeValueAsString(data)
}

