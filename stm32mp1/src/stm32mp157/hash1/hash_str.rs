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
#[doc = "Field `NBLW` reader - NBLW"]
pub type NBLW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NBLW` writer - NBLW"]
pub type NBLW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HASH_STR_SPEC, u8, u8, 5, O>;
#[doc = "Field `DCAL` writer - DCAL"]
pub type DCAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, HASH_STR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:4 - NBLW"]
    #[inline(always)]
    pub fn nblw(&self) -> NBLW_R {
        NBLW_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - NBLW"]
    #[inline(always)]
    #[must_use]
    pub fn nblw(&mut self) -> NBLW_W<0> {
        NBLW_W::new(self)
    }
    #[doc = "Bit 8 - DCAL"]
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
#[doc = "The HASH_STR register has two functions: It is used to define the number of valid bits in the last word of the message entered in the hash processor (that is the number of valid least significant bits in the last data written to the HASH_DIN register) It is used to start the processing of the last block in the message by writing the DCAL bit to 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_str](index.html) module"]
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
