#[doc = "Register `TAMP_SCR` writer"]
pub struct W(crate::W<TAMP_SCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TAMP_SCR_SPEC>;
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
impl From<crate::W<TAMP_SCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TAMP_SCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTAMP1F` writer - Clear TAMP1 detection flag Writing 1 in this bit clears the TAMP1F bit in the TAMP_SR register."]
pub type CTAMP1F_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMP_SCR_SPEC, bool, O>;
#[doc = "Field `CTAMP2F` writer - Clear TAMP2 detection flag Writing 1 in this bit clears the TAMP2F bit in the TAMP_SR register."]
pub type CTAMP2F_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMP_SCR_SPEC, bool, O>;
#[doc = "Field `CTAMP3F` writer - Clear TAMP3 detection flag Writing 1 in this bit clears the TAMP3F bit in the TAMP_SR register."]
pub type CTAMP3F_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMP_SCR_SPEC, bool, O>;
#[doc = "Field `CTAMP4F` writer - Clear TAMP4 detection flag Writing 1 in this bit clears the TAMP4F bit in the TAMP_SR register."]
pub type CTAMP4F_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMP_SCR_SPEC, bool, O>;
#[doc = "Field `CTAMP5F` writer - Clear TAMP5 detection flag Writing 1 in this bit clears the TAMP5F bit in the TAMP_SR register."]
pub type CTAMP5F_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMP_SCR_SPEC, bool, O>;
#[doc = "Field `CTAMP6F` writer - Clear TAMP6 detection flag Writing 1 in this bit clears the TAMP6F bit in the TAMP_SR register."]
pub type CTAMP6F_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMP_SCR_SPEC, bool, O>;
#[doc = "Field `CTAMP7F` writer - Clear TAMP7 detection flag Writing 1 in this bit clears the TAMP7F bit in the TAMP_SR register."]
pub type CTAMP7F_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMP_SCR_SPEC, bool, O>;
#[doc = "Field `CTAMP8F` writer - Clear TAMP8 detection flag Writing 1 in this bit clears the TAMP8F bit in the TAMP_SR register."]
pub type CTAMP8F_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMP_SCR_SPEC, bool, O>;
#[doc = "Field `CITAMP1F` writer - Clear ITAMP1 detection flag Writing 1 in this bit clears the ITAMP1F bit in the TAMP_SR register."]
pub type CITAMP1F_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMP_SCR_SPEC, bool, O>;
#[doc = "Field `CITAMP2F` writer - Clear ITAMP2 detection flag Writing 1 in this bit clears the ITAMP2F bit in the TAMP_SR register."]
pub type CITAMP2F_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMP_SCR_SPEC, bool, O>;
#[doc = "Field `CITAMP3F` writer - Clear ITAMP3 detection flag Writing 1 in this bit clears the ITAMP3F bit in the TAMP_SR register."]
pub type CITAMP3F_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMP_SCR_SPEC, bool, O>;
#[doc = "Field `CITAMP4F` writer - Clear ITAMP4 detection flag Writing 1 in this bit clears the ITAMP4F bit in the TAMP_SR register."]
pub type CITAMP4F_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMP_SCR_SPEC, bool, O>;
#[doc = "Field `CITAMP5F` writer - Clear ITAMP5 detection flag Writing 1 in this bit clears the ITAMP5F bit in the TAMP_SR register."]
pub type CITAMP5F_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMP_SCR_SPEC, bool, O>;
#[doc = "Field `CITAMP6F` writer - Clear ITAMP6 detection flag Writing 1 in this bit clears the ITAMP6F bit in the TAMP_SR register."]
pub type CITAMP6F_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMP_SCR_SPEC, bool, O>;
#[doc = "Field `CITAMP7F` writer - Clear ITAMP7 detection flag Writing 1 in this bit clears the ITAMP7F bit in the TAMP_SR register."]
pub type CITAMP7F_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMP_SCR_SPEC, bool, O>;
#[doc = "Field `CITAMP8F` writer - Clear ITAMP8 detection flag Writing 1 in this bit clears the ITAMP8F bit in the TAMP_SR register."]
pub type CITAMP8F_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMP_SCR_SPEC, bool, O>;
#[doc = "Field `CITAMP9F` writer - Clear ITAMP9 detection flag Writing 1 in this bit clears the ITAMP9F bit in the TAMP_SR register."]
pub type CITAMP9F_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMP_SCR_SPEC, bool, O>;
#[doc = "Field `CITAMP11F` writer - Clear ITAMP11 detection flag Writing 1 in this bit clears the ITAMP11F bit in the TAMP_SR register."]
pub type CITAMP11F_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMP_SCR_SPEC, bool, O>;
#[doc = "Field `CITAMP12F` writer - Clear ITAMP12 detection flag Writing 1 in this bit clears the ITAMP12F bit in the TAMP_SR register."]
pub type CITAMP12F_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMP_SCR_SPEC, bool, O>;
#[doc = "Field `CITAMP13F` writer - Clear ITAMP13 detection flag Writing 1 in this bit clears the ITAMP13F bit in the TAMP_SR register."]
pub type CITAMP13F_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMP_SCR_SPEC, bool, O>;
#[doc = "Field `CITAMP15F` writer - Clear ITAMP15 detection flag Writing 1 in this bit clears the ITAMP15F bit in the TAMP_SR register."]
pub type CITAMP15F_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMP_SCR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Clear TAMP1 detection flag Writing 1 in this bit clears the TAMP1F bit in the TAMP_SR register."]
    #[inline(always)]
    #[must_use]
    pub fn ctamp1f(&mut self) -> CTAMP1F_W<0> {
        CTAMP1F_W::new(self)
    }
    #[doc = "Bit 1 - Clear TAMP2 detection flag Writing 1 in this bit clears the TAMP2F bit in the TAMP_SR register."]
    #[inline(always)]
    #[must_use]
    pub fn ctamp2f(&mut self) -> CTAMP2F_W<1> {
        CTAMP2F_W::new(self)
    }
    #[doc = "Bit 2 - Clear TAMP3 detection flag Writing 1 in this bit clears the TAMP3F bit in the TAMP_SR register."]
    #[inline(always)]
    #[must_use]
    pub fn ctamp3f(&mut self) -> CTAMP3F_W<2> {
        CTAMP3F_W::new(self)
    }
    #[doc = "Bit 3 - Clear TAMP4 detection flag Writing 1 in this bit clears the TAMP4F bit in the TAMP_SR register."]
    #[inline(always)]
    #[must_use]
    pub fn ctamp4f(&mut self) -> CTAMP4F_W<3> {
        CTAMP4F_W::new(self)
    }
    #[doc = "Bit 4 - Clear TAMP5 detection flag Writing 1 in this bit clears the TAMP5F bit in the TAMP_SR register."]
    #[inline(always)]
    #[must_use]
    pub fn ctamp5f(&mut self) -> CTAMP5F_W<4> {
        CTAMP5F_W::new(self)
    }
    #[doc = "Bit 5 - Clear TAMP6 detection flag Writing 1 in this bit clears the TAMP6F bit in the TAMP_SR register."]
    #[inline(always)]
    #[must_use]
    pub fn ctamp6f(&mut self) -> CTAMP6F_W<5> {
        CTAMP6F_W::new(self)
    }
    #[doc = "Bit 6 - Clear TAMP7 detection flag Writing 1 in this bit clears the TAMP7F bit in the TAMP_SR register."]
    #[inline(always)]
    #[must_use]
    pub fn ctamp7f(&mut self) -> CTAMP7F_W<6> {
        CTAMP7F_W::new(self)
    }
    #[doc = "Bit 7 - Clear TAMP8 detection flag Writing 1 in this bit clears the TAMP8F bit in the TAMP_SR register."]
    #[inline(always)]
    #[must_use]
    pub fn ctamp8f(&mut self) -> CTAMP8F_W<7> {
        CTAMP8F_W::new(self)
    }
    #[doc = "Bit 16 - Clear ITAMP1 detection flag Writing 1 in this bit clears the ITAMP1F bit in the TAMP_SR register."]
    #[inline(always)]
    #[must_use]
    pub fn citamp1f(&mut self) -> CITAMP1F_W<16> {
        CITAMP1F_W::new(self)
    }
    #[doc = "Bit 17 - Clear ITAMP2 detection flag Writing 1 in this bit clears the ITAMP2F bit in the TAMP_SR register."]
    #[inline(always)]
    #[must_use]
    pub fn citamp2f(&mut self) -> CITAMP2F_W<17> {
        CITAMP2F_W::new(self)
    }
    #[doc = "Bit 18 - Clear ITAMP3 detection flag Writing 1 in this bit clears the ITAMP3F bit in the TAMP_SR register."]
    #[inline(always)]
    #[must_use]
    pub fn citamp3f(&mut self) -> CITAMP3F_W<18> {
        CITAMP3F_W::new(self)
    }
    #[doc = "Bit 19 - Clear ITAMP4 detection flag Writing 1 in this bit clears the ITAMP4F bit in the TAMP_SR register."]
    #[inline(always)]
    #[must_use]
    pub fn citamp4f(&mut self) -> CITAMP4F_W<19> {
        CITAMP4F_W::new(self)
    }
    #[doc = "Bit 20 - Clear ITAMP5 detection flag Writing 1 in this bit clears the ITAMP5F bit in the TAMP_SR register."]
    #[inline(always)]
    #[must_use]
    pub fn citamp5f(&mut self) -> CITAMP5F_W<20> {
        CITAMP5F_W::new(self)
    }
    #[doc = "Bit 21 - Clear ITAMP6 detection flag Writing 1 in this bit clears the ITAMP6F bit in the TAMP_SR register."]
    #[inline(always)]
    #[must_use]
    pub fn citamp6f(&mut self) -> CITAMP6F_W<21> {
        CITAMP6F_W::new(self)
    }
    #[doc = "Bit 22 - Clear ITAMP7 detection flag Writing 1 in this bit clears the ITAMP7F bit in the TAMP_SR register."]
    #[inline(always)]
    #[must_use]
    pub fn citamp7f(&mut self) -> CITAMP7F_W<22> {
        CITAMP7F_W::new(self)
    }
    #[doc = "Bit 23 - Clear ITAMP8 detection flag Writing 1 in this bit clears the ITAMP8F bit in the TAMP_SR register."]
    #[inline(always)]
    #[must_use]
    pub fn citamp8f(&mut self) -> CITAMP8F_W<23> {
        CITAMP8F_W::new(self)
    }
    #[doc = "Bit 24 - Clear ITAMP9 detection flag Writing 1 in this bit clears the ITAMP9F bit in the TAMP_SR register."]
    #[inline(always)]
    #[must_use]
    pub fn citamp9f(&mut self) -> CITAMP9F_W<24> {
        CITAMP9F_W::new(self)
    }
    #[doc = "Bit 26 - Clear ITAMP11 detection flag Writing 1 in this bit clears the ITAMP11F bit in the TAMP_SR register."]
    #[inline(always)]
    #[must_use]
    pub fn citamp11f(&mut self) -> CITAMP11F_W<26> {
        CITAMP11F_W::new(self)
    }
    #[doc = "Bit 27 - Clear ITAMP12 detection flag Writing 1 in this bit clears the ITAMP12F bit in the TAMP_SR register."]
    #[inline(always)]
    #[must_use]
    pub fn citamp12f(&mut self) -> CITAMP12F_W<27> {
        CITAMP12F_W::new(self)
    }
    #[doc = "Bit 28 - Clear ITAMP13 detection flag Writing 1 in this bit clears the ITAMP13F bit in the TAMP_SR register."]
    #[inline(always)]
    #[must_use]
    pub fn citamp13f(&mut self) -> CITAMP13F_W<28> {
        CITAMP13F_W::new(self)
    }
    #[doc = "Bit 30 - Clear ITAMP15 detection flag Writing 1 in this bit clears the ITAMP15F bit in the TAMP_SR register."]
    #[inline(always)]
    #[must_use]
    pub fn citamp15f(&mut self) -> CITAMP15F_W<30> {
        CITAMP15F_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TAMP status clear register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tamp_scr](index.html) module"]
pub struct TAMP_SCR_SPEC;
impl crate::RegisterSpec for TAMP_SCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [tamp_scr::W](W) writer structure"]
impl crate::Writable for TAMP_SCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TAMP_SCR to value 0"]
impl crate::Resettable for TAMP_SCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
