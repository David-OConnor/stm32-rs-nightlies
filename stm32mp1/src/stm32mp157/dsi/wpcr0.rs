#[doc = "Register `WPCR0` reader"]
pub struct R(crate::R<WPCR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WPCR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WPCR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WPCR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WPCR0` writer"]
pub struct W(crate::W<WPCR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WPCR0_SPEC>;
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
impl From<crate::W<WPCR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WPCR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UIX4` reader - UIX4"]
pub type UIX4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UIX4` writer - UIX4"]
pub type UIX4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WPCR0_SPEC, u8, u8, 6, O>;
#[doc = "Field `SWCL` reader - SWCL"]
pub type SWCL_R = crate::BitReader<bool>;
#[doc = "Field `SWCL` writer - SWCL"]
pub type SWCL_W<'a, const O: u8> = crate::BitWriter<'a, u32, WPCR0_SPEC, bool, O>;
#[doc = "Field `SWDL0` reader - SWDL0"]
pub type SWDL0_R = crate::BitReader<bool>;
#[doc = "Field `SWDL0` writer - SWDL0"]
pub type SWDL0_W<'a, const O: u8> = crate::BitWriter<'a, u32, WPCR0_SPEC, bool, O>;
#[doc = "Field `SWDL1` reader - SWDL1"]
pub type SWDL1_R = crate::BitReader<bool>;
#[doc = "Field `SWDL1` writer - SWDL1"]
pub type SWDL1_W<'a, const O: u8> = crate::BitWriter<'a, u32, WPCR0_SPEC, bool, O>;
#[doc = "Field `HSICL` reader - HSICL"]
pub type HSICL_R = crate::BitReader<bool>;
#[doc = "Field `HSICL` writer - HSICL"]
pub type HSICL_W<'a, const O: u8> = crate::BitWriter<'a, u32, WPCR0_SPEC, bool, O>;
#[doc = "Field `HSIDL0` reader - HSIDL0"]
pub type HSIDL0_R = crate::BitReader<bool>;
#[doc = "Field `HSIDL0` writer - HSIDL0"]
pub type HSIDL0_W<'a, const O: u8> = crate::BitWriter<'a, u32, WPCR0_SPEC, bool, O>;
#[doc = "Field `HSIDL1` reader - HSIDL1"]
pub type HSIDL1_R = crate::BitReader<bool>;
#[doc = "Field `HSIDL1` writer - HSIDL1"]
pub type HSIDL1_W<'a, const O: u8> = crate::BitWriter<'a, u32, WPCR0_SPEC, bool, O>;
#[doc = "Field `FTXSMCL` reader - FTXSMCL"]
pub type FTXSMCL_R = crate::BitReader<bool>;
#[doc = "Field `FTXSMCL` writer - FTXSMCL"]
pub type FTXSMCL_W<'a, const O: u8> = crate::BitWriter<'a, u32, WPCR0_SPEC, bool, O>;
#[doc = "Field `FTXSMDL` reader - FTXSMDL"]
pub type FTXSMDL_R = crate::BitReader<bool>;
#[doc = "Field `FTXSMDL` writer - FTXSMDL"]
pub type FTXSMDL_W<'a, const O: u8> = crate::BitWriter<'a, u32, WPCR0_SPEC, bool, O>;
#[doc = "Field `CDOFFDL` reader - CDOFFDL"]
pub type CDOFFDL_R = crate::BitReader<bool>;
#[doc = "Field `CDOFFDL` writer - CDOFFDL"]
pub type CDOFFDL_W<'a, const O: u8> = crate::BitWriter<'a, u32, WPCR0_SPEC, bool, O>;
#[doc = "Field `TDDL` reader - TDDL"]
pub type TDDL_R = crate::BitReader<bool>;
#[doc = "Field `TDDL` writer - TDDL"]
pub type TDDL_W<'a, const O: u8> = crate::BitWriter<'a, u32, WPCR0_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:5 - UIX4"]
    #[inline(always)]
    pub fn uix4(&self) -> UIX4_R {
        UIX4_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - SWCL"]
    #[inline(always)]
    pub fn swcl(&self) -> SWCL_R {
        SWCL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SWDL0"]
    #[inline(always)]
    pub fn swdl0(&self) -> SWDL0_R {
        SWDL0_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SWDL1"]
    #[inline(always)]
    pub fn swdl1(&self) -> SWDL1_R {
        SWDL1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - HSICL"]
    #[inline(always)]
    pub fn hsicl(&self) -> HSICL_R {
        HSICL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - HSIDL0"]
    #[inline(always)]
    pub fn hsidl0(&self) -> HSIDL0_R {
        HSIDL0_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - HSIDL1"]
    #[inline(always)]
    pub fn hsidl1(&self) -> HSIDL1_R {
        HSIDL1_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - FTXSMCL"]
    #[inline(always)]
    pub fn ftxsmcl(&self) -> FTXSMCL_R {
        FTXSMCL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - FTXSMDL"]
    #[inline(always)]
    pub fn ftxsmdl(&self) -> FTXSMDL_R {
        FTXSMDL_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - CDOFFDL"]
    #[inline(always)]
    pub fn cdoffdl(&self) -> CDOFFDL_R {
        CDOFFDL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - TDDL"]
    #[inline(always)]
    pub fn tddl(&self) -> TDDL_R {
        TDDL_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - UIX4"]
    #[inline(always)]
    #[must_use]
    pub fn uix4(&mut self) -> UIX4_W<0> {
        UIX4_W::new(self)
    }
    #[doc = "Bit 6 - SWCL"]
    #[inline(always)]
    #[must_use]
    pub fn swcl(&mut self) -> SWCL_W<6> {
        SWCL_W::new(self)
    }
    #[doc = "Bit 7 - SWDL0"]
    #[inline(always)]
    #[must_use]
    pub fn swdl0(&mut self) -> SWDL0_W<7> {
        SWDL0_W::new(self)
    }
    #[doc = "Bit 8 - SWDL1"]
    #[inline(always)]
    #[must_use]
    pub fn swdl1(&mut self) -> SWDL1_W<8> {
        SWDL1_W::new(self)
    }
    #[doc = "Bit 9 - HSICL"]
    #[inline(always)]
    #[must_use]
    pub fn hsicl(&mut self) -> HSICL_W<9> {
        HSICL_W::new(self)
    }
    #[doc = "Bit 10 - HSIDL0"]
    #[inline(always)]
    #[must_use]
    pub fn hsidl0(&mut self) -> HSIDL0_W<10> {
        HSIDL0_W::new(self)
    }
    #[doc = "Bit 11 - HSIDL1"]
    #[inline(always)]
    #[must_use]
    pub fn hsidl1(&mut self) -> HSIDL1_W<11> {
        HSIDL1_W::new(self)
    }
    #[doc = "Bit 12 - FTXSMCL"]
    #[inline(always)]
    #[must_use]
    pub fn ftxsmcl(&mut self) -> FTXSMCL_W<12> {
        FTXSMCL_W::new(self)
    }
    #[doc = "Bit 13 - FTXSMDL"]
    #[inline(always)]
    #[must_use]
    pub fn ftxsmdl(&mut self) -> FTXSMDL_W<13> {
        FTXSMDL_W::new(self)
    }
    #[doc = "Bit 14 - CDOFFDL"]
    #[inline(always)]
    #[must_use]
    pub fn cdoffdl(&mut self) -> CDOFFDL_W<14> {
        CDOFFDL_W::new(self)
    }
    #[doc = "Bit 16 - TDDL"]
    #[inline(always)]
    #[must_use]
    pub fn tddl(&mut self) -> TDDL_W<16> {
        TDDL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI wrapper PHY configuration register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wpcr0](index.html) module"]
pub struct WPCR0_SPEC;
impl crate::RegisterSpec for WPCR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wpcr0::R](R) reader structure"]
impl crate::Readable for WPCR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wpcr0::W](W) writer structure"]
impl crate::Writable for WPCR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WPCR0 to value 0"]
impl crate::Resettable for WPCR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
