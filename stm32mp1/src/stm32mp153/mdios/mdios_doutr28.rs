#[doc = "Register `MDIOS_DOUTR28` reader"]
pub struct R(crate::R<MDIOS_DOUTR28_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDIOS_DOUTR28_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDIOS_DOUTR28_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDIOS_DOUTR28_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DOUT` reader - DOUT"]
pub type DOUT_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - DOUT"]
    #[inline(always)]
    pub fn dout(&self) -> DOUT_R {
        DOUT_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MDIOS output data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdios_doutr28](index.html) module"]
pub struct MDIOS_DOUTR28_SPEC;
impl crate::RegisterSpec for MDIOS_DOUTR28_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mdios_doutr28::R](R) reader structure"]
impl crate::Readable for MDIOS_DOUTR28_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MDIOS_DOUTR28 to value 0"]
impl crate::Resettable for MDIOS_DOUTR28_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
