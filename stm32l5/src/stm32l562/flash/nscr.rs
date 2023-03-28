#[doc = "Register `NSCR` reader"]
pub struct R(crate::R<NSCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NSCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NSCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NSCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NSCR` writer"]
pub struct W(crate::W<NSCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NSCR_SPEC>;
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
impl From<crate::W<NSCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NSCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NSPG` reader - NSPG"]
pub type NSPG_R = crate::BitReader<bool>;
#[doc = "Field `NSPG` writer - NSPG"]
pub type NSPG_W<'a, const O: u8> = crate::BitWriter<'a, u32, NSCR_SPEC, bool, O>;
#[doc = "Field `NSPER` reader - NSPER"]
pub type NSPER_R = crate::BitReader<bool>;
#[doc = "Field `NSPER` writer - NSPER"]
pub type NSPER_W<'a, const O: u8> = crate::BitWriter<'a, u32, NSCR_SPEC, bool, O>;
#[doc = "Field `NSMER1` reader - NSMER1"]
pub type NSMER1_R = crate::BitReader<bool>;
#[doc = "Field `NSMER1` writer - NSMER1"]
pub type NSMER1_W<'a, const O: u8> = crate::BitWriter<'a, u32, NSCR_SPEC, bool, O>;
#[doc = "Field `NSPNB` reader - NSPNB"]
pub type NSPNB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NSPNB` writer - NSPNB"]
pub type NSPNB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NSCR_SPEC, u8, u8, 7, O>;
#[doc = "Field `NSBKER` reader - NSBKER"]
pub type NSBKER_R = crate::BitReader<bool>;
#[doc = "Field `NSBKER` writer - NSBKER"]
pub type NSBKER_W<'a, const O: u8> = crate::BitWriter<'a, u32, NSCR_SPEC, bool, O>;
#[doc = "Field `NSMER2` reader - NSMER2"]
pub type NSMER2_R = crate::BitReader<bool>;
#[doc = "Field `NSMER2` writer - NSMER2"]
pub type NSMER2_W<'a, const O: u8> = crate::BitWriter<'a, u32, NSCR_SPEC, bool, O>;
#[doc = "Field `NSSTRT` reader - Options modification start"]
pub type NSSTRT_R = crate::BitReader<bool>;
#[doc = "Field `NSSTRT` writer - Options modification start"]
pub type NSSTRT_W<'a, const O: u8> = crate::BitWriter<'a, u32, NSCR_SPEC, bool, O>;
#[doc = "Field `OPTSTRT` reader - Options modification start"]
pub type OPTSTRT_R = crate::BitReader<bool>;
#[doc = "Field `OPTSTRT` writer - Options modification start"]
pub type OPTSTRT_W<'a, const O: u8> = crate::BitWriter<'a, u32, NSCR_SPEC, bool, O>;
#[doc = "Field `NSEOPIE` reader - NSEOPIE"]
pub type NSEOPIE_R = crate::BitReader<bool>;
#[doc = "Field `NSEOPIE` writer - NSEOPIE"]
pub type NSEOPIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, NSCR_SPEC, bool, O>;
#[doc = "Field `NSERRIE` reader - NSERRIE"]
pub type NSERRIE_R = crate::BitReader<bool>;
#[doc = "Field `NSERRIE` writer - NSERRIE"]
pub type NSERRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, NSCR_SPEC, bool, O>;
#[doc = "Field `OBL_LAUNCH` reader - Force the option byte loading"]
pub type OBL_LAUNCH_R = crate::BitReader<bool>;
#[doc = "Field `OBL_LAUNCH` writer - Force the option byte loading"]
pub type OBL_LAUNCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, NSCR_SPEC, bool, O>;
#[doc = "Field `OPTLOCK` reader - Options Lock"]
pub type OPTLOCK_R = crate::BitReader<bool>;
#[doc = "Field `OPTLOCK` writer - Options Lock"]
pub type OPTLOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, NSCR_SPEC, bool, O>;
#[doc = "Field `NSLOCK` reader - NSLOCK"]
pub type NSLOCK_R = crate::BitReader<bool>;
#[doc = "Field `NSLOCK` writer - NSLOCK"]
pub type NSLOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, NSCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - NSPG"]
    #[inline(always)]
    pub fn nspg(&self) -> NSPG_R {
        NSPG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NSPER"]
    #[inline(always)]
    pub fn nsper(&self) -> NSPER_R {
        NSPER_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NSMER1"]
    #[inline(always)]
    pub fn nsmer1(&self) -> NSMER1_R {
        NSMER1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:9 - NSPNB"]
    #[inline(always)]
    pub fn nspnb(&self) -> NSPNB_R {
        NSPNB_R::new(((self.bits >> 3) & 0x7f) as u8)
    }
    #[doc = "Bit 11 - NSBKER"]
    #[inline(always)]
    pub fn nsbker(&self) -> NSBKER_R {
        NSBKER_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - NSMER2"]
    #[inline(always)]
    pub fn nsmer2(&self) -> NSMER2_R {
        NSMER2_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Options modification start"]
    #[inline(always)]
    pub fn nsstrt(&self) -> NSSTRT_R {
        NSSTRT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Options modification start"]
    #[inline(always)]
    pub fn optstrt(&self) -> OPTSTRT_R {
        OPTSTRT_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 24 - NSEOPIE"]
    #[inline(always)]
    pub fn nseopie(&self) -> NSEOPIE_R {
        NSEOPIE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - NSERRIE"]
    #[inline(always)]
    pub fn nserrie(&self) -> NSERRIE_R {
        NSERRIE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - Force the option byte loading"]
    #[inline(always)]
    pub fn obl_launch(&self) -> OBL_LAUNCH_R {
        OBL_LAUNCH_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 30 - Options Lock"]
    #[inline(always)]
    pub fn optlock(&self) -> OPTLOCK_R {
        OPTLOCK_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - NSLOCK"]
    #[inline(always)]
    pub fn nslock(&self) -> NSLOCK_R {
        NSLOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - NSPG"]
    #[inline(always)]
    #[must_use]
    pub fn nspg(&mut self) -> NSPG_W<0> {
        NSPG_W::new(self)
    }
    #[doc = "Bit 1 - NSPER"]
    #[inline(always)]
    #[must_use]
    pub fn nsper(&mut self) -> NSPER_W<1> {
        NSPER_W::new(self)
    }
    #[doc = "Bit 2 - NSMER1"]
    #[inline(always)]
    #[must_use]
    pub fn nsmer1(&mut self) -> NSMER1_W<2> {
        NSMER1_W::new(self)
    }
    #[doc = "Bits 3:9 - NSPNB"]
    #[inline(always)]
    #[must_use]
    pub fn nspnb(&mut self) -> NSPNB_W<3> {
        NSPNB_W::new(self)
    }
    #[doc = "Bit 11 - NSBKER"]
    #[inline(always)]
    #[must_use]
    pub fn nsbker(&mut self) -> NSBKER_W<11> {
        NSBKER_W::new(self)
    }
    #[doc = "Bit 15 - NSMER2"]
    #[inline(always)]
    #[must_use]
    pub fn nsmer2(&mut self) -> NSMER2_W<15> {
        NSMER2_W::new(self)
    }
    #[doc = "Bit 16 - Options modification start"]
    #[inline(always)]
    #[must_use]
    pub fn nsstrt(&mut self) -> NSSTRT_W<16> {
        NSSTRT_W::new(self)
    }
    #[doc = "Bit 17 - Options modification start"]
    #[inline(always)]
    #[must_use]
    pub fn optstrt(&mut self) -> OPTSTRT_W<17> {
        OPTSTRT_W::new(self)
    }
    #[doc = "Bit 24 - NSEOPIE"]
    #[inline(always)]
    #[must_use]
    pub fn nseopie(&mut self) -> NSEOPIE_W<24> {
        NSEOPIE_W::new(self)
    }
    #[doc = "Bit 25 - NSERRIE"]
    #[inline(always)]
    #[must_use]
    pub fn nserrie(&mut self) -> NSERRIE_W<25> {
        NSERRIE_W::new(self)
    }
    #[doc = "Bit 27 - Force the option byte loading"]
    #[inline(always)]
    #[must_use]
    pub fn obl_launch(&mut self) -> OBL_LAUNCH_W<27> {
        OBL_LAUNCH_W::new(self)
    }
    #[doc = "Bit 30 - Options Lock"]
    #[inline(always)]
    #[must_use]
    pub fn optlock(&mut self) -> OPTLOCK_W<30> {
        OPTLOCK_W::new(self)
    }
    #[doc = "Bit 31 - NSLOCK"]
    #[inline(always)]
    #[must_use]
    pub fn nslock(&mut self) -> NSLOCK_W<31> {
        NSLOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash non-secure control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nscr](index.html) module"]
pub struct NSCR_SPEC;
impl crate::RegisterSpec for NSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nscr::R](R) reader structure"]
impl crate::Readable for NSCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nscr::W](W) writer structure"]
impl crate::Writable for NSCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NSCR to value 0xc000_0000"]
impl crate::Resettable for NSCR_SPEC {
    const RESET_VALUE: Self::Ux = 0xc000_0000;
}
