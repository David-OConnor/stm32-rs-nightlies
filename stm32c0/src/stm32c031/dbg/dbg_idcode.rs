#[doc = "Register `DBG_IDCODE` reader"]
pub struct R(crate::R<DBG_IDCODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DBG_IDCODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DBG_IDCODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DBG_IDCODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DEV_ID` reader - Device identifier This bitfield indicates the device ID."]
pub type DEV_ID_R = crate::FieldReader<u16, u16>;
#[doc = "Field `REV_ID` reader - Revision identifier This bitfield indicates the revision of the device."]
pub type REV_ID_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:11 - Device identifier This bitfield indicates the device ID."]
    #[inline(always)]
    pub fn dev_id(&self) -> DEV_ID_R {
        DEV_ID_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:31 - Revision identifier This bitfield indicates the revision of the device."]
    #[inline(always)]
    pub fn rev_id(&self) -> REV_ID_R {
        REV_ID_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "DBG device ID code register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbg_idcode](index.html) module"]
pub struct DBG_IDCODE_SPEC;
impl crate::RegisterSpec for DBG_IDCODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dbg_idcode::R](R) reader structure"]
impl crate::Readable for DBG_IDCODE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DBG_IDCODE to value 0x1000_0453"]
impl crate::Resettable for DBG_IDCODE_SPEC {
    const RESET_VALUE: Self::Ux = 0x1000_0453;
}
