#[doc = "Register `I3C_RDWR` reader"]
pub struct R(crate::R<I3C_RDWR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I3C_RDWR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I3C_RDWR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I3C_RDWR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RDB0` reader - 8-bit received data (earliest byte on I3C bus)."]
pub type RDB0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RDB1` reader - 8-bit received data (next byte after RDB0 on I3C bus)."]
pub type RDB1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RDB2` reader - 8-bit received data (next byte after RDB1 on I3C bus)."]
pub type RDB2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RDB3` reader - 8-bit received data (latest byte on I3C bus)."]
pub type RDB3_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - 8-bit received data (earliest byte on I3C bus)."]
    #[inline(always)]
    pub fn rdb0(&self) -> RDB0_R {
        RDB0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 8-bit received data (next byte after RDB0 on I3C bus)."]
    #[inline(always)]
    pub fn rdb1(&self) -> RDB1_R {
        RDB1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 8-bit received data (next byte after RDB1 on I3C bus)."]
    #[inline(always)]
    pub fn rdb2(&self) -> RDB2_R {
        RDB2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 8-bit received data (latest byte on I3C bus)."]
    #[inline(always)]
    pub fn rdb3(&self) -> RDB3_R {
        RDB3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "I3C receive data word register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i3c_rdwr](index.html) module"]
pub struct I3C_RDWR_SPEC;
impl crate::RegisterSpec for I3C_RDWR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i3c_rdwr::R](R) reader structure"]
impl crate::Readable for I3C_RDWR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets I3C_RDWR to value 0"]
impl crate::Resettable for I3C_RDWR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
