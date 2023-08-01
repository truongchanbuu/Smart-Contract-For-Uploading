# Ứng dụng "Copyright System"

## Giới thiệu

Ứng dụng "Copyright System" được xây dựng dựa trên blockchain của NEAR Protocol. Ứng dụng này cho phép các tác giả đăng ký và bảo vệ các tác phẩm của họ trên blockchain. Nó cung cấp các chức năng cho tác giả để tạo, cập nhật và xóa các tác phẩm, và cho phép người dùng khác truy cập và đánh giá các tác phẩm. Ngoài ra, còn có một frontend tạm thời để sử dụng các chức năng như thêm tác phẩm vào smart contract hoặc xóa tác phẩm.

## Ưu điểm

- **Bảo mật**: Các thông tin về tác phẩm được lưu trữ trên blockchain, đảm bảo tính bất biến và không thể thay đổi dễ dàng.
- **Phân quyền**: Tác giả có quyền kiểm soát các tác phẩm của mình và quyết định về việc công khai tác phẩm hay chỉ cho phép một số người được truy cập.
- **Phân chia lợi nhuận**: Hệ thống tự động phân chia lợi nhuận từ các tác phẩm đăng tải giữa tác giả và các cộng tác viên dựa trên tỷ lệ đã được định sẵn.

## Nhược điểm

- **Giao diện người dùng chưa phát triển đầy đủ**: Giao diện người dùng có thể cần cải thiện để cung cấp trải nghiệm tốt hơn cho người dùng cuối.
- **Chi phí giao dịch**: Giao dịch trên blockchain đòi hỏi phải trả phí, điều này có thể tạo ra một số chi phí phụ thuộc vào kích thước và phức tạp của giao dịch.

## Các chức năng

1. **Tạo tác giả mới (`create_author`)**: Cho phép tạo mới một tác giả với thông tin như tên, tuổi và danh sách tác phẩm được đánh giá.
2. **Tạo tác phẩm mới (`create_work`)**: Cho phép tạo tác phẩm mới với thông tin như tên, nội dung, danh sách cộng tác viên, phí, tỷ lệ chia lợi nhuận.
3. **Cập nhật thông tin tác giả (`update_author`)**: Cho phép tác giả cập nhật thông tin cá nhân như tên và tuổi.
4. **Cập nhật thông tin tác phẩm (`update_work`)**: Cho phép tác giả cập nhật thông tin của tác phẩm, bao gồm tên, nội dung, phí, tỷ lệ chia lợi nhuận và điểm đánh giá trung bình.
5. **Xóa tác phẩm (`delete_work`)**: Cho phép tác giả xóa một tác phẩm đã đăng tải. Nếu muốn xóa thì cần có sự chấp thuận của từ 75% thành viên trở lên hoặc là chính tác giả nếu có bằng hoặc dưới 2 người thông qua hàm `vote`.
6. **Xóa tác giả (`delete_author`)**: Cho phép xóa tài khoản tác giả.
7. **Xem thông tin tác phẩm (`get_work_by_id`)**: Cho phép xem thông tin chi tiết của một tác phẩm dựa trên ID của nó. Nếu không phải là tác giả hoặc cộng tác viên thì cần phải lấy quyền truy cập thuộc tính `content` của tác phẩm để xem được tác phẩm.
8. **Xem danh sách tác giả (`get_all_authors`)**: Cho phép xem danh sách tất cả tác giả đã đăng ký trong hệ thống.
9. **Xem danh sách tác phẩm của tác giả (`get_all_works_of_author`)**: Cho phép xem danh sách tất cả tác phẩm của một tác giả cụ thể. Nếu không phải là tác giả hoặc cộng tác viên thì cần phải lấy quyền truy cập thuộc tính `content` của tác phẩm để xem được tác phẩm.
10. **Xem danh sách tất cả tác phẩm (`get_all_works`)**: Cho phép xem danh sách tất cả tác phẩm đã được đăng tải trong hệ thống. Nếu không phải là tác giả hoặc cộng tác viên thì cần phải lấy quyền truy cập thuộc tính `content` của tác phẩm để xem được tác phẩm.
11. **Đánh giá tác phẩm (`rate_work`)**: Cho phép người dùng đánh giá tác phẩm với điểm số từ 1 đến 5.
12. **Thêm cộng tác viên (`add_collaborator`)**: Cho phép tác giả thêm cộng tác viên vào tác phẩm của mình. Nếu muốn thêm thành viên cần phải thực hiện hàm `vote` để lấy ý kiến từ tác giả và các cộng tác viên.
13. **Báo cáo vi phạm (`report_infringement`)**: Cho phép người dùng báo cáo vi phạm về tác phẩm.
14. **Phân phối lợi nhuận (`distribute_funds`)**: Cho phép phân phối lợi nhuận từ tác phẩm cho các tác giả và cộng tác viên theo tỷ lệ
