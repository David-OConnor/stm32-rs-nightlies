#[doc = "Register `EECR3` reader"]
pub struct R(crate::R<EECR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EECR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EECR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EECR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EECR3` writer"]
pub struct W(crate::W<EECR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EECR3_SPEC>;
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
impl From<crate::W<EECR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EECR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EE6F` reader - EE6F"]
pub type EE6F_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EE6F` writer - EE6F"]
pub type EE6F_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EECR3_SPEC, u8, u8, 4, O>;
#[doc = "Field `EE7F` reader - EE7F"]
pub type EE7F_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EE7F` writer - EE7F"]
pub type EE7F_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EECR3_SPEC, u8, u8, 4, O>;
#[doc = "Field `EE8F` reader - EE8F"]
pub type EE8F_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EE8F` writer - EE8F"]
pub type EE8F_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EECR3_SPEC, u8, u8, 4, O>;
#[doc = "Field `EE9F` reader - EE9F"]
pub type EE9F_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EE9F` writer - EE9F"]
pub type EE9F_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EECR3_SPEC, u8, u8, 4, O>;
#[doc = "Field `EE10F` reader - EE10F"]
pub type EE10F_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EE10F` writer - EE10F"]
pub type EE10F_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EECR3_SPEC, u8, u8, 4, O>;
#[doc = "Field `EEVSD` reader - EEVSD"]
pub type EEVSD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EEVSD` writer - EEVSD"]
pub type EEVSD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EECR3_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:3 - EE6F"]
    #[inline(always)]
    pub fn ee6f(&self) -> EE6F_R {
        EE6F_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 6:9 - EE7F"]
    #[inline(always)]
    pub fn ee7f(&self) -> EE7F_R {
        EE7F_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - EE8F"]
    #[inline(always)]
    pub fn ee8f(&self) -> EE8F_R {
        EE8F_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 18:21 - EE9F"]
    #[inline(always)]
    pub fn ee9f(&self) -> EE9F_R {
        EE9F_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - EE10F"]
    #[inline(always)]
    pub fn ee10f(&self) -> EE10F_R {
        EE10F_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 30:31 - EEVSD"]
    #[inline(always)]
    pub fn eevsd(&self) -> EEVSD_R {
        EEVSD_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - EE6F"]
    #[inline(always)]
    #[must_use]
    pub fn ee6f(&mut self) -> EE6F_W<0> {
        EE6F_W::new(self)
    }
    #[doc = "Bits 6:9 - EE7F"]
    #[inline(always)]
    #[must_use]
    pub fn ee7f(&mut self) -> EE7F_W<6> {
        EE7F_W::new(self)
    }
    #[doc = "Bits 12:15 - EE8F"]
    #[inline(always)]
    #[must_use]
    pub fn ee8f(&mut self) -> EE8F_W<12> {
        EE8F_W::new(self)
    }
    #[doc = "Bits 18:21 - EE9F"]
    #[inline(always)]
    #[must_use]
    pub fn ee9f(&mut self) -> EE9F_W<18> {
        EE9F_W::new(self)
    }
    #[doc = "Bits 24:27 - EE10F"]
    #[inline(always)]
    #[must_use]
    pub fn ee10f(&mut self) -> EE10F_W<24> {
        EE10F_W::new(self)
    }
    #[doc = "Bits 30:31 - EEVSD"]
    #[inline(always)]
    #[must_use]
    pub fn eevsd(&mut self) -> EEVSD_W<30> {
        EEVSD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer External Event Control Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eecr3](index.html) module"]
pub struct EECR3_SPEC;
impl crate::RegisterSpec for EECR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eecr3::R](R) reader structure"]
impl crate::Readable for EECR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eecr3::W](W) writer structure"]
impl crate::Writable for EECR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EECR3 to value 0"]
impl crate::Resettable for EECR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
