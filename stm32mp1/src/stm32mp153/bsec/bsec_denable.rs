#[doc = "Register `BSEC_DENABLE` reader"]
pub struct R(crate::R<BSEC_DENABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BSEC_DENABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BSEC_DENABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BSEC_DENABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BSEC_DENABLE` writer"]
pub struct W(crate::W<BSEC_DENABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BSEC_DENABLE_SPEC>;
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
impl From<crate::W<BSEC_DENABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BSEC_DENABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DFTEN` reader - DFTEN"]
pub type DFTEN_R = crate::BitReader<bool>;
#[doc = "Field `DFTEN` writer - DFTEN"]
pub type DFTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BSEC_DENABLE_SPEC, bool, O>;
#[doc = "Field `DBGEN` reader - DBGEN"]
pub type DBGEN_R = crate::BitReader<bool>;
#[doc = "Field `DBGEN` writer - DBGEN"]
pub type DBGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BSEC_DENABLE_SPEC, bool, O>;
#[doc = "Field `NIDEN` reader - NIDEN"]
pub type NIDEN_R = crate::BitReader<bool>;
#[doc = "Field `NIDEN` writer - NIDEN"]
pub type NIDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BSEC_DENABLE_SPEC, bool, O>;
#[doc = "Field `DEVICEEN` reader - DEVICEEN"]
pub type DEVICEEN_R = crate::BitReader<bool>;
#[doc = "Field `DEVICEEN` writer - DEVICEEN"]
pub type DEVICEEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BSEC_DENABLE_SPEC, bool, O>;
#[doc = "Field `HDPEN` reader - HDPEN"]
pub type HDPEN_R = crate::BitReader<bool>;
#[doc = "Field `HDPEN` writer - HDPEN"]
pub type HDPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BSEC_DENABLE_SPEC, bool, O>;
#[doc = "Field `SPIDEN` reader - SPIDEN"]
pub type SPIDEN_R = crate::BitReader<bool>;
#[doc = "Field `SPIDEN` writer - SPIDEN"]
pub type SPIDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BSEC_DENABLE_SPEC, bool, O>;
#[doc = "Field `SPNIDEN` reader - SPNIDEN"]
pub type SPNIDEN_R = crate::BitReader<bool>;
#[doc = "Field `SPNIDEN` writer - SPNIDEN"]
pub type SPNIDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BSEC_DENABLE_SPEC, bool, O>;
#[doc = "Field `CP15SDISABLE` reader - CP15SDISABLE"]
pub type CP15SDISABLE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CP15SDISABLE` writer - CP15SDISABLE"]
pub type CP15SDISABLE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BSEC_DENABLE_SPEC, u8, u8, 2, O>;
#[doc = "Field `CFGSDISABLE` reader - CFGSDISABLE"]
pub type CFGSDISABLE_R = crate::BitReader<bool>;
#[doc = "Field `CFGSDISABLE` writer - CFGSDISABLE"]
pub type CFGSDISABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, BSEC_DENABLE_SPEC, bool, O>;
#[doc = "Field `DBGSWENABLE` reader - DBGSWENABLE"]
pub type DBGSWENABLE_R = crate::BitReader<bool>;
#[doc = "Field `DBGSWENABLE` writer - DBGSWENABLE"]
pub type DBGSWENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, BSEC_DENABLE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - DFTEN"]
    #[inline(always)]
    pub fn dften(&self) -> DFTEN_R {
        DFTEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DBGEN"]
    #[inline(always)]
    pub fn dbgen(&self) -> DBGEN_R {
        DBGEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NIDEN"]
    #[inline(always)]
    pub fn niden(&self) -> NIDEN_R {
        NIDEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DEVICEEN"]
    #[inline(always)]
    pub fn deviceen(&self) -> DEVICEEN_R {
        DEVICEEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - HDPEN"]
    #[inline(always)]
    pub fn hdpen(&self) -> HDPEN_R {
        HDPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SPIDEN"]
    #[inline(always)]
    pub fn spiden(&self) -> SPIDEN_R {
        SPIDEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SPNIDEN"]
    #[inline(always)]
    pub fn spniden(&self) -> SPNIDEN_R {
        SPNIDEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:8 - CP15SDISABLE"]
    #[inline(always)]
    pub fn cp15sdisable(&self) -> CP15SDISABLE_R {
        CP15SDISABLE_R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bit 9 - CFGSDISABLE"]
    #[inline(always)]
    pub fn cfgsdisable(&self) -> CFGSDISABLE_R {
        CFGSDISABLE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DBGSWENABLE"]
    #[inline(always)]
    pub fn dbgswenable(&self) -> DBGSWENABLE_R {
        DBGSWENABLE_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DFTEN"]
    #[inline(always)]
    #[must_use]
    pub fn dften(&mut self) -> DFTEN_W<0> {
        DFTEN_W::new(self)
    }
    #[doc = "Bit 1 - DBGEN"]
    #[inline(always)]
    #[must_use]
    pub fn dbgen(&mut self) -> DBGEN_W<1> {
        DBGEN_W::new(self)
    }
    #[doc = "Bit 2 - NIDEN"]
    #[inline(always)]
    #[must_use]
    pub fn niden(&mut self) -> NIDEN_W<2> {
        NIDEN_W::new(self)
    }
    #[doc = "Bit 3 - DEVICEEN"]
    #[inline(always)]
    #[must_use]
    pub fn deviceen(&mut self) -> DEVICEEN_W<3> {
        DEVICEEN_W::new(self)
    }
    #[doc = "Bit 4 - HDPEN"]
    #[inline(always)]
    #[must_use]
    pub fn hdpen(&mut self) -> HDPEN_W<4> {
        HDPEN_W::new(self)
    }
    #[doc = "Bit 5 - SPIDEN"]
    #[inline(always)]
    #[must_use]
    pub fn spiden(&mut self) -> SPIDEN_W<5> {
        SPIDEN_W::new(self)
    }
    #[doc = "Bit 6 - SPNIDEN"]
    #[inline(always)]
    #[must_use]
    pub fn spniden(&mut self) -> SPNIDEN_W<6> {
        SPNIDEN_W::new(self)
    }
    #[doc = "Bits 7:8 - CP15SDISABLE"]
    #[inline(always)]
    #[must_use]
    pub fn cp15sdisable(&mut self) -> CP15SDISABLE_W<7> {
        CP15SDISABLE_W::new(self)
    }
    #[doc = "Bit 9 - CFGSDISABLE"]
    #[inline(always)]
    #[must_use]
    pub fn cfgsdisable(&mut self) -> CFGSDISABLE_W<9> {
        CFGSDISABLE_W::new(self)
    }
    #[doc = "Bit 10 - DBGSWENABLE"]
    #[inline(always)]
    #[must_use]
    pub fn dbgswenable(&mut self) -> DBGSWENABLE_W<10> {
        DBGSWENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "reset value depends on OTP secure mode according toTable18: BSEC_DENABLE default values after reset on page181.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_denable](index.html) module"]
pub struct BSEC_DENABLE_SPEC;
impl crate::RegisterSpec for BSEC_DENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bsec_denable::R](R) reader structure"]
impl crate::Readable for BSEC_DENABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bsec_denable::W](W) writer structure"]
impl crate::Writable for BSEC_DENABLE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BSEC_DENABLE to value 0"]
impl crate::Resettable for BSEC_DENABLE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
