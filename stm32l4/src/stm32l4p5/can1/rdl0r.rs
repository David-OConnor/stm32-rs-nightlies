#[doc = "Register `RDL0R` reader"]
pub struct R(crate::R<RDL0R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RDL0R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RDL0R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RDL0R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATA0` reader - DATA0"]
pub type DATA0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA1` reader - DATA1"]
pub type DATA1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA2` reader - DATA2"]
pub type DATA2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA3` reader - DATA3"]
pub type DATA3_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - DATA0"]
    #[inline(always)]
    pub fn data0(&self) -> DATA0_R {
        DATA0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - DATA1"]
    #[inline(always)]
    pub fn data1(&self) -> DATA1_R {
        DATA1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - DATA2"]
    #[inline(always)]
    pub fn data2(&self) -> DATA2_R {
        DATA2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - DATA3"]
    #[inline(always)]
    pub fn data3(&self) -> DATA3_R {
        DATA3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "mailbox data high register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rdl0r](index.html) module"]
pub struct RDL0R_SPEC;
impl crate::RegisterSpec for RDL0R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rdl0r::R](R) reader structure"]
impl crate::Readable for RDL0R_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RDL0R to value 0"]
impl crate::Resettable for RDL0R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
