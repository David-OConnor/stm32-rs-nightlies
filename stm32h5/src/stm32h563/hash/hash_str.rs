#[doc = "Register `HASH_STR` reader"]
pub struct R(crate::R<HASH_STR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HASH_STR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HASH_STR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HASH_STR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HASH_STR` writer"]
pub struct W(crate::W<HASH_STR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HASH_STR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<HASH_STR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HASH_STR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NBLW` reader - Number of valid bits in the last word When the last word of the message bit string is written to HASH_DIN register, the hash processor takes only the valid bits, specified as below, after internal data swapping: ... The above mechanism is valid only if DCAL = 0. If NBLW bits are written while DCAL is set to 1, the NBLW bitfield remains unchanged. In other words it is not possible to configure NBLW and set DCAL at the same time. Reading NBLW bits returns the last value written to NBLW."]
pub type NBLW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NBLW` writer - Number of valid bits in the last word When the last word of the message bit string is written to HASH_DIN register, the hash processor takes only the valid bits, specified as below, after internal data swapping: ... The above mechanism is valid only if DCAL = 0. If NBLW bits are written while DCAL is set to 1, the NBLW bitfield remains unchanged. In other words it is not possible to configure NBLW and set DCAL at the same time. Reading NBLW bits returns the last value written to NBLW."]
pub type NBLW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HASH_STR_SPEC, u8, u8, 5, O>;
#[doc = "Field `DCAL` reader - Digest calculation Writing this bit to 1 starts the message padding using the previously written value of NBLW, and starts the calculation of the final message digest with all the data words written to the input FIFO since the INIT bit was last written to 1. Reading this bit returns 0."]
pub type DCAL_R = crate::BitReader<bool>;
#[doc = "Field `DCAL` writer - Digest calculation Writing this bit to 1 starts the message padding using the previously written value of NBLW, and starts the calculation of the final message digest with all the data words written to the input FIFO since the INIT bit was last written to 1. Reading this bit returns 0."]
pub type DCAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, HASH_STR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:4 - Number of valid bits in the last word When the last word of the message bit string is written to HASH_DIN register, the hash processor takes only the valid bits, specified as below, after internal data swapping: ... The above mechanism is valid only if DCAL = 0. If NBLW bits are written while DCAL is set to 1, the NBLW bitfield remains unchanged. In other words it is not possible to configure NBLW and set DCAL at the same time. Reading NBLW bits returns the last value written to NBLW."]
    #[inline(always)]
    pub fn nblw(&self) -> NBLW_R {
        NBLW_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 8 - Digest calculation Writing this bit to 1 starts the message padding using the previously written value of NBLW, and starts the calculation of the final message digest with all the data words written to the input FIFO since the INIT bit was last written to 1. Reading this bit returns 0."]
    #[inline(always)]
    pub fn dcal(&self) -> DCAL_R {
        DCAL_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Number of valid bits in the last word When the last word of the message bit string is written to HASH_DIN register, the hash processor takes only the valid bits, specified as below, after internal data swapping: ... The above mechanism is valid only if DCAL = 0. If NBLW bits are written while DCAL is set to 1, the NBLW bitfield remains unchanged. In other words it is not possible to configure NBLW and set DCAL at the same time. Reading NBLW bits returns the last value written to NBLW."]
    #[inline(always)]
    #[must_use]
    pub fn nblw(&mut self) -> NBLW_W<0> {
        NBLW_W::new(self)
    }
    #[doc = "Bit 8 - Digest calculation Writing this bit to 1 starts the message padding using the previously written value of NBLW, and starts the calculation of the final message digest with all the data words written to the input FIFO since the INIT bit was last written to 1. Reading this bit returns 0."]
    #[inline(always)]
    #[must_use]
    pub fn dcal(&mut self) -> DCAL_W<8> {
        DCAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HASH start register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_str](index.html) module"]
pub struct HASH_STR_SPEC;
impl crate::RegisterSpec for HASH_STR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hash_str::R](R) reader structure"]
impl crate::Readable for HASH_STR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hash_str::W](W) writer structure"]
impl crate::Writable for HASH_STR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HASH_STR to value 0"]
impl crate::Resettable for HASH_STR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
