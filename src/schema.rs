table! {
    admin_auto_recommendations (id) {
        id -> Integer,
        project_id -> Integer,
        small_tag_id -> Integer,
        recommended_project_id -> Integer,
        recommended_kind -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    article_article_serials (article_id, article_serial_id) {
        article_id -> Integer,
        article_serial_id -> Integer,
        sort -> Integer,
    }
}

table! {
    article_filters (id) {
        id -> Integer,
        legacy_tag_id -> Integer,
        is_displayed -> Bool,
        sort -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    article_images (id) {
        id -> Integer,
        article_id -> Integer,
        image_type -> Varchar,
        path -> Varchar,
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    article_serial_filters (id) {
        id -> Integer,
        legacy_tag_id -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    article_serial_images (id) {
        id -> Integer,
        article_serial_id -> Integer,
        image_type -> Varchar,
        path -> Varchar,
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    article_serials (id) {
        id -> Integer,
        name -> Varchar,
        description -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    article_serials_legacy_tags (article_serial_id, legacy_tag_id) {
        article_serial_id -> Integer,
        legacy_tag_id -> Integer,
    }
}

table! {
    articles (id) {
        id -> Integer,
        user_id -> Integer,
        title -> Varchar,
        content -> Mediumtext,
        summary -> Varchar,
        status -> Varchar,
        is_login_required -> Bool,
        publish_from -> Nullable<Datetime>,
        publish_until -> Nullable<Datetime>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    articles_legacy_tags (article_id, legacy_tag_id) {
        article_id -> Integer,
        legacy_tag_id -> Integer,
    }
}

table! {
    articles_projects (article_id, project_id) {
        article_id -> Integer,
        project_id -> Integer,
    }
}

table! {
    bag_images (id) {
        id -> Integer,
        bag_id -> Integer,
        image_type -> Varchar,
        path -> Varchar,
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    bags (id) {
        id -> Integer,
        identifier -> Varchar,
        name -> Varchar,
        description -> Varchar,
        meta_keywords -> Varchar,
        meta_description -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    bags_projects (bag_id, project_id) {
        bag_id -> Integer,
        project_id -> Integer,
    }
}

table! {
    banner_images (id) {
        id -> Integer,
        banner_id -> Integer,
        image_type -> Varchar,
        name -> Varchar,
        link -> Varchar,
        path -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    banners (id) {
        id -> Integer,
        name -> Varchar,
        state -> Varchar,
        sort -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    blog_filters (id) {
        id -> Integer,
        legacy_tag_id -> Integer,
        is_displayed -> Bool,
        sort -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    blog_images (id) {
        id -> Integer,
        blog_id -> Integer,
        image_type -> Varchar,
        path -> Varchar,
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    blogs (id) {
        id -> Integer,
        user_id -> Integer,
        title -> Varchar,
        content -> Mediumtext,
        summary -> Varchar,
        status -> Varchar,
        publish_from -> Nullable<Datetime>,
        publish_until -> Nullable<Datetime>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    blogs_legacy_tags (blog_id, legacy_tag_id) {
        blog_id -> Integer,
        legacy_tag_id -> Integer,
    }
}

table! {
    blogs_projects (blog_id, project_id) {
        blog_id -> Integer,
        project_id -> Integer,
    }
}

table! {
    form_templates (id) {
        id -> Integer,
        form -> Json,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    inquiries (id) {
        id -> Integer,
        project_id -> Nullable<Integer>,
        inquiry_type -> Varchar,
        name -> Varchar,
        email -> Varchar,
        message -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    joif_forms (id) {
        id -> Integer,
        organization_id -> Nullable<Integer>,
        plan -> Varchar,
        company_class -> Varchar,
        payment_method -> Varchar,
        company_name -> Varchar,
        job_name -> Varchar,
        user_name -> Varchar,
        email -> Varchar,
        postcode -> Varchar,
        address1 -> Varchar,
        address2 -> Varchar,
        price -> Integer,
        stripe_customer -> Varchar,
        invoice_company_name -> Varchar,
        invoice_phone_number -> Varchar,
        invoice_email -> Varchar,
        invoice_representative -> Varchar,
        invoice_user_name -> Varchar,
        invoice_postcode -> Varchar,
        invoice_address1 -> Varchar,
        invoice_address2 -> Varchar,
        is_provided_information -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    laboratories (id) {
        id -> Integer,
        laboratory_group_id -> Integer,
        name -> Varchar,
        sort -> Integer,
        location -> Varchar,
        schedule -> Datetime,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    laboratories_legacy_tags (laboratory_id, legacy_tag_id) {
        laboratory_id -> Integer,
        legacy_tag_id -> Integer,
    }
}

table! {
    laboratory_groups (id) {
        id -> Integer,
        path -> Varchar,
        name -> Varchar,
        shorten -> Varchar,
        explain -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    laboratory_images (id) {
        id -> Integer,
        laboratory_id -> Integer,
        image_type -> Varchar,
        path -> Varchar,
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    laboratory_links (id) {
        id -> Integer,
        laboratory_id -> Integer,
        link_type -> Varchar,
        link -> Varchar,
        name -> Varchar,
        explain -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    large_tags (id) {
        id -> Integer,
        name -> Varchar,
        sort -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    legacy_tags (id) {
        id -> Integer,
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    medium_tags (id) {
        id -> Integer,
        large_tag_id -> Integer,
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    organization_images (id) {
        id -> Integer,
        organization_id -> Integer,
        image_type -> Varchar,
        path -> Varchar,
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    organization_plan_buyers (id) {
        id -> Integer,
        company_name -> Varchar,
        phone_number -> Varchar,
        email -> Varchar,
        representative -> Varchar,
        user_name -> Varchar,
        postcode -> Varchar,
        address1 -> Varchar,
        address2 -> Varchar,
        invoice_postcode -> Varchar,
        invoice_address1 -> Varchar,
        invoice_address2 -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    organization_plans (id) {
        id -> Integer,
        organization_id -> Integer,
        organization_plan_buyer_id -> Integer,
        plan -> Varchar,
        payment_method -> Varchar,
        stripe_customer -> Varchar,
        stripe_subscription -> Varchar,
        unsubscribed_reason -> Varchar,
        unsubscribed_requested_at -> Nullable<Datetime>,
        unsubscribed_actually_at -> Nullable<Datetime>,
        is_pending -> Bool,
        is_enabled -> Nullable<Bool>,
        contract_price -> Integer,
        contract_version -> Integer,
        contract_period -> Integer,
        contract_updated_at -> Nullable<Datetime>,
        contract_started_at -> Nullable<Datetime>,
        contract_expired_at -> Datetime,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    organization_profiles (id) {
        id -> Integer,
        organization_id -> Integer,
        capital_fund -> Nullable<Varchar>,
        number_of_employee -> Nullable<Varchar>,
        listed -> Nullable<Varchar>,
        stage -> Nullable<Varchar>,
        capital_type -> Nullable<Varchar>,
        name -> Varchar,
        name_ruby -> Varchar,
        url -> Varchar,
        address -> Varchar,
        organizer -> Varchar,
        establishment_year -> Nullable<Integer>,
        sales_year -> Nullable<Integer>,
        sales_value -> Nullable<Bigint>,
        average_age -> Nullable<Integer>,
        major_customers -> Varchar,
        overview -> Varchar,
        shareholders -> Varchar,
        recommendation -> Varchar,
        phone_number -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    organizations (id) {
        id -> Integer,
        parent_id -> Nullable<Integer>,
        identifier -> Varchar,
        role -> Varchar,
        is_authed -> Bool,
        is_published -> Bool,
        is_deleted -> Bool,
        deleted_at -> Nullable<Datetime>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    poster_images (id) {
        id -> Integer,
        poster_id -> Integer,
        image_type -> Varchar,
        path -> Varchar,
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    posters (id) {
        id -> Integer,
        name -> Varchar,
        description -> Varchar,
        link -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    project_contact_messages (id) {
        id -> Integer,
        project_contact_id -> Integer,
        from_id -> Integer,
        to_id -> Integer,
        message -> Text,
        is_read -> Bool,
        read_at -> Nullable<Datetime>,
        is_deleted -> Bool,
        deleted_at -> Nullable<Datetime>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    project_contacts (id) {
        id -> Integer,
        sender_id -> Integer,
        recipient_id -> Integer,
        sender_status -> Varchar,
        recipient_status -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    project_division_filters (id) {
        id -> Integer,
        legacy_tag_id -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    project_environment_filters (id) {
        id -> Integer,
        legacy_tag_id -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    project_events (id) {
        id -> Integer,
        from_id -> Nullable<Integer>,
        to_id -> Nullable<Integer>,
        event_type -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    project_favorites (id) {
        id -> Integer,
        project_id -> Integer,
        favorite_id -> Integer,
        content -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    project_feature_filters (id) {
        id -> Integer,
        legacy_tag_id -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    project_form_content_images (id) {
        id -> Integer,
        project_form_content_id -> Integer,
        image_type -> Varchar,
        name -> Varchar,
        path -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    project_form_contents (id) {
        id -> Integer,
        project_form_id -> Integer,
        postedby_id -> Integer,
        status -> Varchar,
        content -> Json,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    project_forms (id) {
        id -> Integer,
        project_id -> Integer,
        form_template_id -> Integer,
        description -> Varchar,
        publish_from -> Nullable<Datetime>,
        publish_until -> Nullable<Datetime>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    project_images (id) {
        id -> Integer,
        project_id -> Integer,
        image_type -> Varchar,
        path -> Varchar,
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    project_innovators (id) {
        id -> Integer,
        project_id -> Integer,
        name -> Varchar,
        position -> Varchar,
        message -> Varchar,
        department -> Varchar,
        description -> Varchar,
        mission -> Varchar,
        career -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    project_knowledge_filters (id) {
        id -> Integer,
        legacy_tag_id -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    project_needs_small_tags (project_id, small_tag_id) {
        project_id -> Integer,
        small_tag_id -> Integer,
    }
}

table! {
    project_notifies (id) {
        id -> Integer,
        project_id -> Integer,
        recommend_id -> Nullable<Integer>,
        recommend_type -> Nullable<Varchar>,
        notify_type -> Varchar,
        title -> Varchar,
        summary -> Varchar,
        content -> Varchar,
        is_read -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    project_other_filters (id) {
        id -> Integer,
        legacy_tag_id -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    project_pickup_filters (id) {
        id -> Integer,
        legacy_tag_id -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    project_pr_small_tags (project_id, small_tag_id) {
        project_id -> Integer,
        small_tag_id -> Integer,
    }
}

table! {
    project_profiles (id) {
        id -> Integer,
        project_id -> Integer,
        title -> Text,
        goal -> Text,
        message -> Text,
        resource -> Text,
        achievement -> Text,
        requirements -> Text,
        description -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    project_service_filters (id) {
        id -> Integer,
        legacy_tag_id -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    project_tickets (id) {
        id -> Integer,
        project_id -> Integer,
        name -> Varchar,
        ticket_type -> Varchar,
        unique_key -> Nullable<Varchar>,
        is_used -> Bool,
        used_at -> Nullable<Datetime>,
        enabled_at -> Nullable<Datetime>,
        expired_at -> Nullable<Datetime>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    projects (id) {
        id -> Integer,
        organization_id -> Integer,
        identifier -> Varchar,
        is_published -> Bool,
        is_deleted -> Bool,
        deleted_at -> Nullable<Datetime>,
        is_contactable -> Bool,
        is_formable -> Bool,
        cheat_score -> Unsigned<Integer>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    projects_legacy_tags (project_id, legacy_tag_id) {
        project_id -> Integer,
        legacy_tag_id -> Integer,
    }
}

table! {
    small_tags (id) {
        id -> Integer,
        medium_tag_id -> Nullable<Integer>,
        name -> Varchar,
        is_authed -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    special_images (id) {
        id -> Integer,
        special_id -> Integer,
        image_type -> Varchar,
        path -> Varchar,
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    specials (id) {
        id -> Integer,
        name -> Varchar,
        description -> Varchar,
        sort -> Integer,
        is_top -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    specials_projects (special_id, project_id) {
        special_id -> Integer,
        project_id -> Integer,
    }
}

table! {
    translators (id) {
        id -> Integer,
        lang -> Varchar,
        orig -> Varchar,
        trans -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    user_activities (id) {
        id -> Integer,
        user_id -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    user_follows (followee_id, follower_id) {
        followee_id -> Integer,
        follower_id -> Integer,
    }
}

table! {
    user_images (id) {
        id -> Integer,
        user_id -> Integer,
        image_type -> Varchar,
        path -> Varchar,
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    user_picks (id) {
        id -> Integer,
        user_id -> Integer,
        document_id -> Integer,
        document_type -> Varchar,
        comment -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    user_profiles (id) {
        id -> Integer,
        user_id -> Integer,
        name -> Varchar,
        company -> Varchar,
        position -> Varchar,
        bio -> Varchar,
        mission -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    user_socials (id) {
        id -> Integer,
        user_id -> Integer,
        provider -> Varchar,
        identifier -> Varchar,
        extra_data -> Json,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Integer,
        who -> Nullable<Varchar>,
        identifier -> Varchar,
        role -> Varchar,
        email -> Varchar,
        password -> Varchar,
        date_joined -> Datetime,
        last_login -> Nullable<Datetime>,
        is_company_receive_unread -> Bool,
        is_company_receive_information -> Bool,
        is_person_receive_information -> Bool,
        is_authed -> Bool,
        is_published -> Bool,
        is_pro -> Bool,
        is_deleted -> Bool,
        deleted_at -> Nullable<Datetime>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    users_legacy_tags (user_id, legacy_tag_id) {
        user_id -> Integer,
        legacy_tag_id -> Integer,
    }
}

table! {
    users_organizations (user_id, organization_id) {
        user_id -> Integer,
        organization_id -> Integer,
    }
}

joinable!(admin_auto_recommendations -> small_tags (small_tag_id));
joinable!(article_article_serials -> article_serials (article_serial_id));
joinable!(article_article_serials -> articles (article_id));
joinable!(article_filters -> legacy_tags (legacy_tag_id));
joinable!(article_images -> articles (article_id));
joinable!(article_serial_filters -> legacy_tags (legacy_tag_id));
joinable!(article_serial_images -> article_serials (article_serial_id));
joinable!(article_serials_legacy_tags -> article_serials (article_serial_id));
joinable!(article_serials_legacy_tags -> legacy_tags (legacy_tag_id));
joinable!(articles -> users (user_id));
joinable!(articles_legacy_tags -> articles (article_id));
joinable!(articles_legacy_tags -> legacy_tags (legacy_tag_id));
joinable!(articles_projects -> articles (article_id));
joinable!(articles_projects -> projects (project_id));
joinable!(bag_images -> bags (bag_id));
joinable!(bags_projects -> bags (bag_id));
joinable!(bags_projects -> projects (project_id));
joinable!(banner_images -> banners (banner_id));
joinable!(blog_filters -> legacy_tags (legacy_tag_id));
joinable!(blog_images -> blogs (blog_id));
joinable!(blogs -> users (user_id));
joinable!(blogs_legacy_tags -> blogs (blog_id));
joinable!(blogs_legacy_tags -> legacy_tags (legacy_tag_id));
joinable!(blogs_projects -> blogs (blog_id));
joinable!(blogs_projects -> projects (project_id));
joinable!(inquiries -> projects (project_id));
joinable!(joif_forms -> organizations (organization_id));
joinable!(laboratories -> laboratory_groups (laboratory_group_id));
joinable!(laboratories_legacy_tags -> laboratories (laboratory_id));
joinable!(laboratories_legacy_tags -> legacy_tags (legacy_tag_id));
joinable!(laboratory_images -> laboratories (laboratory_id));
joinable!(laboratory_links -> laboratories (laboratory_id));
joinable!(medium_tags -> large_tags (large_tag_id));
joinable!(organization_images -> organizations (organization_id));
joinable!(organization_plans -> organization_plan_buyers (organization_plan_buyer_id));
joinable!(organization_plans -> organizations (organization_id));
joinable!(organization_profiles -> organizations (organization_id));
joinable!(poster_images -> posters (poster_id));
joinable!(project_contact_messages -> project_contacts (project_contact_id));
joinable!(project_division_filters -> legacy_tags (legacy_tag_id));
joinable!(project_environment_filters -> legacy_tags (legacy_tag_id));
joinable!(project_feature_filters -> legacy_tags (legacy_tag_id));
joinable!(project_form_content_images -> project_form_contents (project_form_content_id));
joinable!(project_form_contents -> project_forms (project_form_id));
joinable!(project_form_contents -> projects (postedby_id));
joinable!(project_forms -> form_templates (form_template_id));
joinable!(project_forms -> projects (project_id));
joinable!(project_images -> projects (project_id));
joinable!(project_innovators -> projects (project_id));
joinable!(project_knowledge_filters -> legacy_tags (legacy_tag_id));
joinable!(project_needs_small_tags -> projects (project_id));
joinable!(project_needs_small_tags -> small_tags (small_tag_id));
joinable!(project_notifies -> projects (project_id));
joinable!(project_other_filters -> legacy_tags (legacy_tag_id));
joinable!(project_pickup_filters -> legacy_tags (legacy_tag_id));
joinable!(project_pr_small_tags -> projects (project_id));
joinable!(project_pr_small_tags -> small_tags (small_tag_id));
joinable!(project_profiles -> projects (project_id));
joinable!(project_service_filters -> legacy_tags (legacy_tag_id));
joinable!(project_tickets -> projects (project_id));
joinable!(projects -> organizations (organization_id));
joinable!(projects_legacy_tags -> legacy_tags (legacy_tag_id));
joinable!(projects_legacy_tags -> projects (project_id));
joinable!(small_tags -> medium_tags (medium_tag_id));
joinable!(special_images -> specials (special_id));
joinable!(specials_projects -> projects (project_id));
joinable!(specials_projects -> specials (special_id));
joinable!(user_activities -> users (user_id));
joinable!(user_images -> users (user_id));
joinable!(user_picks -> users (user_id));
joinable!(user_profiles -> users (user_id));
joinable!(user_socials -> users (user_id));
joinable!(users_legacy_tags -> legacy_tags (legacy_tag_id));
joinable!(users_legacy_tags -> users (user_id));
joinable!(users_organizations -> organizations (organization_id));
joinable!(users_organizations -> users (user_id));

allow_tables_to_appear_in_same_query!(
    admin_auto_recommendations,
    article_article_serials,
    article_filters,
    article_images,
    article_serial_filters,
    article_serial_images,
    article_serials,
    article_serials_legacy_tags,
    articles,
    articles_legacy_tags,
    articles_projects,
    bag_images,
    bags,
    bags_projects,
    banner_images,
    banners,
    blog_filters,
    blog_images,
    blogs,
    blogs_legacy_tags,
    blogs_projects,
    form_templates,
    inquiries,
    joif_forms,
    laboratories,
    laboratories_legacy_tags,
    laboratory_groups,
    laboratory_images,
    laboratory_links,
    large_tags,
    legacy_tags,
    medium_tags,
    organization_images,
    organization_plan_buyers,
    organization_plans,
    organization_profiles,
    organizations,
    poster_images,
    posters,
    project_contact_messages,
    project_contacts,
    project_division_filters,
    project_environment_filters,
    project_events,
    project_favorites,
    project_feature_filters,
    project_form_content_images,
    project_form_contents,
    project_forms,
    project_images,
    project_innovators,
    project_knowledge_filters,
    project_needs_small_tags,
    project_notifies,
    project_other_filters,
    project_pickup_filters,
    project_pr_small_tags,
    project_profiles,
    project_service_filters,
    project_tickets,
    projects,
    projects_legacy_tags,
    small_tags,
    special_images,
    specials,
    specials_projects,
    translators,
    user_activities,
    user_follows,
    user_images,
    user_picks,
    user_profiles,
    user_socials,
    users,
    users_legacy_tags,
    users_organizations,
);
