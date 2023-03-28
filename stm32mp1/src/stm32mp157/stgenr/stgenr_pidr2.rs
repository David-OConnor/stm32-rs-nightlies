#[doc = "Register `STGENR_PIDR2` reader"]
pub struct R(crate::R<STGENR_PIDR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STGENR_PIDR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STGENR_PIDR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STGENR_PIDR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DES_1` reader - DES_1"]
pub type DES_1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `JEDEC` reader - JEDEC"]
pub type JEDEC_R = crate::BitReader<bool>;
#[doc = "Field `REVISION` reader - REVISION"]
pub type REVISION_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:2 - DES_1"]
    #[inline(always)]
    pub fn des_1(&self) -> DES_1_R {
        DES_1_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - JEDEC"]
    #[inline(always)]
    pub fn jedec(&self) -> JEDEC_R {
        JEDEC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - REVISION"]
    #[inline(always)]
    pub fn revision(&self) -> REVISION_R {
        REVISION_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "STGENR peripheral ID2 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stgenr_pidr2](index.html) module"]
pub struct STGENR_PIDR2_SPEC;
impl crate::RegisterSpec for STGENR_PIDR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stgenr_pidr2::R](R) reader structure"]
impl crate::Readable for STGENR_PIDR2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STGENR_PIDR2 to value 0x1b"]
impl crate::Resettable for STGENR_PIDR2_SPEC {
    const RESET_VALUE: Self::Ux = 0x1b;
}
