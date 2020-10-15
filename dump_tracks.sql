select distinct
  detail_id as id,
  first_value(object_id) over (partition by detail_id) as track_id,
  title as title,
  artist as artist,
  album as album,
  album_art as album_art_id,
  duration as duration,
  date as date,
  mime as mime_type,
  track as track_number,
  disc as disc_number
from objects
join details on details.id=objects.detail_id
where objects.class='item.audioItem.musicTrack';
