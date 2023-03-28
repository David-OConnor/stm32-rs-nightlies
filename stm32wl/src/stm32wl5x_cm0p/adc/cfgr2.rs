#[doc = "Register `CFGR2` reader"]
pub struct R(crate::R<CFGR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGR2` writer"]
pub struct W(crate::W<CFGR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR2_SPEC>;
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
impl From<crate::W<CFGR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OVSE` reader - OVSE"]
pub type OVSE_R = crate::BitReader<bool>;
#[doc = "Field `OVSE` writer - OVSE"]
pub type OVSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, bool, O>;
#[doc = "Field `OVSR0` reader - OVSR0"]
pub type OVSR0_R = crate::BitReader<bool>;
#[doc = "Field `OVSR0` writer - OVSR0"]
pub type OVSR0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, bool, O>;
#[doc = "Field `OVSR1` reader - OVSR1"]
pub type OVSR1_R = crate::BitReader<bool>;
#[doc = "Field `OVSR1` writer - OVSR1"]
pub type OVSR1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, bool, O>;
#[doc = "Field `OVSR2` reader - OVSR2"]
pub type OVSR2_R = crate::BitReader<bool>;
#[doc = "Field `OVSR2` writer - OVSR2"]
pub type OVSR2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, bool, O>;
#[doc = "Field `OVSS0` reader - OVSS0"]
pub type OVSS0_R = crate::BitReader<bool>;
#[doc = "Field `OVSS0` writer - OVSS0"]
pub type OVSS0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, bool, O>;
#[doc = "Field `OVSS1` reader - OVSS1"]
pub type OVSS1_R = crate::BitReader<bool>;
#[doc = "Field `OVSS1` writer - OVSS1"]
pub type OVSS1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, bool, O>;
#[doc = "Field `OVSS2` reader - OVSS2"]
pub type OVSS2_R = crate::BitReader<bool>;
#[doc = "Field `OVSS2` writer - OVSS2"]
pub type OVSS2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, bool, O>;
#[doc = "Field `OVSS3` reader - OVSS3"]
pub type OVSS3_R = crate::BitReader<bool>;
#[doc = "Field `OVSS3` writer - OVSS3"]
pub type OVSS3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, bool, O>;
#[doc = "Field `TOVS` reader - TOVS"]
pub type TOVS_R = crate::BitReader<bool>;
#[doc = "Field `TOVS` writer - TOVS"]
pub type TOVS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, bool, O>;
#[doc = "Field `LFTRIG` reader - LFTRIG"]
pub type LFTRIG_R = crate::BitReader<bool>;
#[doc = "Field `LFTRIG` writer - LFTRIG"]
pub type LFTRIG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, bool, O>;
#[doc = "Field `CKMODE` reader - CKMODE"]
pub type CKMODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CKMODE` writer - CKMODE"]
pub type CKMODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR2_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0 - OVSE"]
    #[inline(always)]
    pub fn ovse(&self) -> OVSE_R {
        OVSE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - OVSR0"]
    #[inline(always)]
    pub fn ovsr0(&self) -> OVSR0_R {
        OVSR0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - OVSR1"]
    #[inline(always)]
    pub fn ovsr1(&self) -> OVSR1_R {
        OVSR1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - OVSR2"]
    #[inline(always)]
    pub fn ovsr2(&self) -> OVSR2_R {
        OVSR2_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - OVSS0"]
    #[inline(always)]
    pub fn ovss0(&self) -> OVSS0_R {
        OVSS0_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - OVSS1"]
    #[inline(always)]
    pub fn ovss1(&self) -> OVSS1_R {
        OVSS1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - OVSS2"]
    #[inline(always)]
    pub fn ovss2(&self) -> OVSS2_R {
        OVSS2_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - OVSS3"]
    #[inline(always)]
    pub fn ovss3(&self) -> OVSS3_R {
        OVSS3_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TOVS"]
    #[inline(always)]
    pub fn tovs(&self) -> TOVS_R {
        TOVS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 29 - LFTRIG"]
    #[inline(always)]
    pub fn lftrig(&self) -> LFTRIG_R {
        LFTRIG_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - CKMODE"]
    #[inline(always)]
    pub fn ckmode(&self) -> CKMODE_R {
        CKMODE_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - OVSE"]
    #[inline(always)]
    #[must_use]
    pub fn ovse(&mut self) -> OVSE_W<0> {
        OVSE_W::new(self)
    }
    #[doc = "Bit 2 - OVSR0"]
    #[inline(always)]
    #[must_use]
    pub fn ovsr0(&mut self) -> OVSR0_W<2> {
        OVSR0_W::new(self)
    }
    #[doc = "Bit 3 - OVSR1"]
    #[inline(always)]
    #[must_use]
    pub fn ovsr1(&mut self) -> OVSR1_W<3> {
        OVSR1_W::new(self)
    }
    #[doc = "Bit 4 - OVSR2"]
    #[inline(always)]
    #[must_use]
    pub fn ovsr2(&mut self) -> OVSR2_W<4> {
        OVSR2_W::new(self)
    }
    #[doc = "Bit 5 - OVSS0"]
    #[inline(always)]
    #[must_use]
    pub fn ovss0(&mut self) -> OVSS0_W<5> {
        OVSS0_W::new(self)
    }
    #[doc = "Bit 6 - OVSS1"]
    #[inline(always)]
    #[must_use]
    pub fn ovss1(&mut self) -> OVSS1_W<6> {
        OVSS1_W::new(self)
    }
    #[doc = "Bit 7 - OVSS2"]
    #[inline(always)]
    #[must_use]
    pub fn ovss2(&mut self) -> OVSS2_W<7> {
        OVSS2_W::new(self)
    }
    #[doc = "Bit 8 - OVSS3"]
    #[inline(always)]
    #[must_use]
    pub fn ovss3(&mut self) -> OVSS3_W<8> {
        OVSS3_W::new(self)
    }
    #[doc = "Bit 9 - TOVS"]
    #[inline(always)]
    #[must_use]
    pub fn tovs(&mut self) -> TOVS_W<9> {
        TOVS_W::new(self)
    }
    #[doc = "Bit 29 - LFTRIG"]
    #[inline(always)]
    #[must_use]
    pub fn lftrig(&mut self) -> LFTRIG_W<29> {
        LFTRIG_W::new(self)
    }
    #[doc = "Bits 30:31 - CKMODE"]
    #[inline(always)]
    #[must_use]
    pub fn ckmode(&mut self) -> CKMODE_W<30> {
        CKMODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr2](index.html) module"]
pub struct CFGR2_SPEC;
impl crate::RegisterSpec for CFGR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgr2::R](R) reader structure"]
impl crate::Readable for CFGR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgr2::W](W) writer structure"]
impl crate::Writable for CFGR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFGR2 to value 0"]
impl crate::Resettable for CFGR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
