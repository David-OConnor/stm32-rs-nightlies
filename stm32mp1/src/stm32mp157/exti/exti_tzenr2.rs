#[doc = "Register `EXTI_TZENR2` reader"]
pub struct R(crate::R<EXTI_TZENR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTI_TZENR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTI_TZENR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTI_TZENR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTI_TZENR2` writer"]
pub struct W(crate::W<EXTI_TZENR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTI_TZENR2_SPEC>;
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
impl From<crate::W<EXTI_TZENR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTI_TZENR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TZEN41` reader - TZEN41"]
pub type TZEN41_R = crate::BitReader<bool>;
#[doc = "Field `TZEN41` writer - TZEN41"]
pub type TZEN41_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_TZENR2_SPEC, bool, O>;
#[doc = "Field `TZEN54` reader - TZEN54"]
pub type TZEN54_R = crate::BitReader<bool>;
#[doc = "Field `TZEN54` writer - TZEN54"]
pub type TZEN54_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_TZENR2_SPEC, bool, O>;
#[doc = "Field `TZEN55` reader - TZEN55"]
pub type TZEN55_R = crate::BitReader<bool>;
#[doc = "Field `TZEN55` writer - TZEN55"]
pub type TZEN55_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_TZENR2_SPEC, bool, O>;
#[doc = "Field `TZEN56` reader - TZEN56"]
pub type TZEN56_R = crate::BitReader<bool>;
#[doc = "Field `TZEN56` writer - TZEN56"]
pub type TZEN56_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_TZENR2_SPEC, bool, O>;
#[doc = "Field `TZEN57` reader - TZEN57"]
pub type TZEN57_R = crate::BitReader<bool>;
#[doc = "Field `TZEN57` writer - TZEN57"]
pub type TZEN57_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_TZENR2_SPEC, bool, O>;
#[doc = "Field `TZEN58` reader - TZEN58"]
pub type TZEN58_R = crate::BitReader<bool>;
#[doc = "Field `TZEN58` writer - TZEN58"]
pub type TZEN58_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_TZENR2_SPEC, bool, O>;
#[doc = "Field `TZEN59` reader - TZEN59"]
pub type TZEN59_R = crate::BitReader<bool>;
#[doc = "Field `TZEN59` writer - TZEN59"]
pub type TZEN59_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_TZENR2_SPEC, bool, O>;
#[doc = "Field `TZEN60` reader - TZEN60"]
pub type TZEN60_R = crate::BitReader<bool>;
#[doc = "Field `TZEN60` writer - TZEN60"]
pub type TZEN60_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_TZENR2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 9 - TZEN41"]
    #[inline(always)]
    pub fn tzen41(&self) -> TZEN41_R {
        TZEN41_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 22 - TZEN54"]
    #[inline(always)]
    pub fn tzen54(&self) -> TZEN54_R {
        TZEN54_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - TZEN55"]
    #[inline(always)]
    pub fn tzen55(&self) -> TZEN55_R {
        TZEN55_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - TZEN56"]
    #[inline(always)]
    pub fn tzen56(&self) -> TZEN56_R {
        TZEN56_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - TZEN57"]
    #[inline(always)]
    pub fn tzen57(&self) -> TZEN57_R {
        TZEN57_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - TZEN58"]
    #[inline(always)]
    pub fn tzen58(&self) -> TZEN58_R {
        TZEN58_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - TZEN59"]
    #[inline(always)]
    pub fn tzen59(&self) -> TZEN59_R {
        TZEN59_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - TZEN60"]
    #[inline(always)]
    pub fn tzen60(&self) -> TZEN60_R {
        TZEN60_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 9 - TZEN41"]
    #[inline(always)]
    #[must_use]
    pub fn tzen41(&mut self) -> TZEN41_W<9> {
        TZEN41_W::new(self)
    }
    #[doc = "Bit 22 - TZEN54"]
    #[inline(always)]
    #[must_use]
    pub fn tzen54(&mut self) -> TZEN54_W<22> {
        TZEN54_W::new(self)
    }
    #[doc = "Bit 23 - TZEN55"]
    #[inline(always)]
    #[must_use]
    pub fn tzen55(&mut self) -> TZEN55_W<23> {
        TZEN55_W::new(self)
    }
    #[doc = "Bit 24 - TZEN56"]
    #[inline(always)]
    #[must_use]
    pub fn tzen56(&mut self) -> TZEN56_W<24> {
        TZEN56_W::new(self)
    }
    #[doc = "Bit 25 - TZEN57"]
    #[inline(always)]
    #[must_use]
    pub fn tzen57(&mut self) -> TZEN57_W<25> {
        TZEN57_W::new(self)
    }
    #[doc = "Bit 26 - TZEN58"]
    #[inline(always)]
    #[must_use]
    pub fn tzen58(&mut self) -> TZEN58_W<26> {
        TZEN58_W::new(self)
    }
    #[doc = "Bit 27 - TZEN59"]
    #[inline(always)]
    #[must_use]
    pub fn tzen59(&mut self) -> TZEN59_W<27> {
        TZEN59_W::new(self)
    }
    #[doc = "Bit 28 - TZEN60"]
    #[inline(always)]
    #[must_use]
    pub fn tzen60(&mut self) -> TZEN60_W<28> {
        TZEN60_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register provides TrustZone Write access security, a non-secure write access will generate a bus error. A non-secure read will return the register data. Contains only register bits for TrustZone capable Input events.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_tzenr2](index.html) module"]
pub struct EXTI_TZENR2_SPEC;
impl crate::RegisterSpec for EXTI_TZENR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [exti_tzenr2::R](R) reader structure"]
impl crate::Readable for EXTI_TZENR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [exti_tzenr2::W](W) writer structure"]
impl crate::Writable for EXTI_TZENR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXTI_TZENR2 to value 0"]
impl crate::Resettable for EXTI_TZENR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
