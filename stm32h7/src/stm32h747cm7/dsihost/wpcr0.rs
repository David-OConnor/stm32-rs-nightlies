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
#[doc = "Field `UIX4` reader - Unit interval multiplied by 4"]
pub type UIX4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UIX4` writer - Unit interval multiplied by 4"]
pub type UIX4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WPCR0_SPEC, u8, u8, 6, O>;
#[doc = "Field `SWCL` reader - Swap clock lane pins"]
pub type SWCL_R = crate::BitReader<bool>;
#[doc = "Field `SWCL` writer - Swap clock lane pins"]
pub type SWCL_W<'a, const O: u8> = crate::BitWriter<'a, u32, WPCR0_SPEC, bool, O>;
#[doc = "Field `SWDL0` reader - Swap data lane 0 pins"]
pub type SWDL0_R = crate::BitReader<bool>;
#[doc = "Field `SWDL0` writer - Swap data lane 0 pins"]
pub type SWDL0_W<'a, const O: u8> = crate::BitWriter<'a, u32, WPCR0_SPEC, bool, O>;
#[doc = "Field `SWDL1` reader - Swap data lane 1 pins"]
pub type SWDL1_R = crate::BitReader<bool>;
#[doc = "Field `SWDL1` writer - Swap data lane 1 pins"]
pub type SWDL1_W<'a, const O: u8> = crate::BitWriter<'a, u32, WPCR0_SPEC, bool, O>;
#[doc = "Field `HSICL` reader - Invert high"]
pub type HSICL_R = crate::BitReader<bool>;
#[doc = "Field `HSICL` writer - Invert high"]
pub type HSICL_W<'a, const O: u8> = crate::BitWriter<'a, u32, WPCR0_SPEC, bool, O>;
#[doc = "Field `HSIDL0` reader - Invert the high-speed data signal on data lane 0"]
pub type HSIDL0_R = crate::BitReader<bool>;
#[doc = "Field `HSIDL0` writer - Invert the high-speed data signal on data lane 0"]
pub type HSIDL0_W<'a, const O: u8> = crate::BitWriter<'a, u32, WPCR0_SPEC, bool, O>;
#[doc = "Field `HSIDL1` reader - Invert the high-speed data signal on data lane 1"]
pub type HSIDL1_R = crate::BitReader<bool>;
#[doc = "Field `HSIDL1` writer - Invert the high-speed data signal on data lane 1"]
pub type HSIDL1_W<'a, const O: u8> = crate::BitWriter<'a, u32, WPCR0_SPEC, bool, O>;
#[doc = "Field `FTXSMCL` reader - Force in TX Stop mode the clock lane"]
pub type FTXSMCL_R = crate::BitReader<bool>;
#[doc = "Field `FTXSMCL` writer - Force in TX Stop mode the clock lane"]
pub type FTXSMCL_W<'a, const O: u8> = crate::BitWriter<'a, u32, WPCR0_SPEC, bool, O>;
#[doc = "Field `FTXSMDL` reader - Force in TX Stop mode the data lanes"]
pub type FTXSMDL_R = crate::BitReader<bool>;
#[doc = "Field `FTXSMDL` writer - Force in TX Stop mode the data lanes"]
pub type FTXSMDL_W<'a, const O: u8> = crate::BitWriter<'a, u32, WPCR0_SPEC, bool, O>;
#[doc = "Field `CDOFFDL` reader - Contention detection OFF on data lanes"]
pub type CDOFFDL_R = crate::BitReader<bool>;
#[doc = "Field `CDOFFDL` writer - Contention detection OFF on data lanes"]
pub type CDOFFDL_W<'a, const O: u8> = crate::BitWriter<'a, u32, WPCR0_SPEC, bool, O>;
#[doc = "Field `TDDL` reader - Turn disable data lanes"]
pub type TDDL_R = crate::BitReader<bool>;
#[doc = "Field `TDDL` writer - Turn disable data lanes"]
pub type TDDL_W<'a, const O: u8> = crate::BitWriter<'a, u32, WPCR0_SPEC, bool, O>;
#[doc = "Field `PDEN` reader - Pull-down enable"]
pub type PDEN_R = crate::BitReader<bool>;
#[doc = "Field `PDEN` writer - Pull-down enable"]
pub type PDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WPCR0_SPEC, bool, O>;
#[doc = "Field `TCLKPREPEN` reader - Custom time for tCLK"]
pub type TCLKPREPEN_R = crate::BitReader<bool>;
#[doc = "Field `TCLKPREPEN` writer - Custom time for tCLK"]
pub type TCLKPREPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WPCR0_SPEC, bool, O>;
#[doc = "Field `TCLKZEROEN` reader - Custom time for tCLK"]
pub type TCLKZEROEN_R = crate::BitReader<bool>;
#[doc = "Field `TCLKZEROEN` writer - Custom time for tCLK"]
pub type TCLKZEROEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WPCR0_SPEC, bool, O>;
#[doc = "Field `THSPREPEN` reader - Custom time for tHS"]
pub type THSPREPEN_R = crate::BitReader<bool>;
#[doc = "Field `THSPREPEN` writer - Custom time for tHS"]
pub type THSPREPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WPCR0_SPEC, bool, O>;
#[doc = "Field `THSTRAILEN` reader - Custom time for tHS"]
pub type THSTRAILEN_R = crate::BitReader<bool>;
#[doc = "Field `THSTRAILEN` writer - Custom time for tHS"]
pub type THSTRAILEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WPCR0_SPEC, bool, O>;
#[doc = "Field `THSZEROEN` reader - Custom time for tHS"]
pub type THSZEROEN_R = crate::BitReader<bool>;
#[doc = "Field `THSZEROEN` writer - Custom time for tHS"]
pub type THSZEROEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WPCR0_SPEC, bool, O>;
#[doc = "Field `TLPXDEN` reader - Custom time for tLPX for data lanes enable"]
pub type TLPXDEN_R = crate::BitReader<bool>;
#[doc = "Field `TLPXDEN` writer - Custom time for tLPX for data lanes enable"]
pub type TLPXDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WPCR0_SPEC, bool, O>;
#[doc = "Field `THSEXITEN` reader - Custom time for tHS"]
pub type THSEXITEN_R = crate::BitReader<bool>;
#[doc = "Field `THSEXITEN` writer - Custom time for tHS"]
pub type THSEXITEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WPCR0_SPEC, bool, O>;
#[doc = "Field `TLPXCEN` reader - Custom time for tLPX for clock lane enable"]
pub type TLPXCEN_R = crate::BitReader<bool>;
#[doc = "Field `TLPXCEN` writer - Custom time for tLPX for clock lane enable"]
pub type TLPXCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WPCR0_SPEC, bool, O>;
#[doc = "Field `TCLKPOSTEN` reader - Custom time for tCLK"]
pub type TCLKPOSTEN_R = crate::BitReader<bool>;
#[doc = "Field `TCLKPOSTEN` writer - Custom time for tCLK"]
pub type TCLKPOSTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WPCR0_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:5 - Unit interval multiplied by 4"]
    #[inline(always)]
    pub fn uix4(&self) -> UIX4_R {
        UIX4_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - Swap clock lane pins"]
    #[inline(always)]
    pub fn swcl(&self) -> SWCL_R {
        SWCL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Swap data lane 0 pins"]
    #[inline(always)]
    pub fn swdl0(&self) -> SWDL0_R {
        SWDL0_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Swap data lane 1 pins"]
    #[inline(always)]
    pub fn swdl1(&self) -> SWDL1_R {
        SWDL1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Invert high"]
    #[inline(always)]
    pub fn hsicl(&self) -> HSICL_R {
        HSICL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Invert the high-speed data signal on data lane 0"]
    #[inline(always)]
    pub fn hsidl0(&self) -> HSIDL0_R {
        HSIDL0_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Invert the high-speed data signal on data lane 1"]
    #[inline(always)]
    pub fn hsidl1(&self) -> HSIDL1_R {
        HSIDL1_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Force in TX Stop mode the clock lane"]
    #[inline(always)]
    pub fn ftxsmcl(&self) -> FTXSMCL_R {
        FTXSMCL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Force in TX Stop mode the data lanes"]
    #[inline(always)]
    pub fn ftxsmdl(&self) -> FTXSMDL_R {
        FTXSMDL_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Contention detection OFF on data lanes"]
    #[inline(always)]
    pub fn cdoffdl(&self) -> CDOFFDL_R {
        CDOFFDL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Turn disable data lanes"]
    #[inline(always)]
    pub fn tddl(&self) -> TDDL_R {
        TDDL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Pull-down enable"]
    #[inline(always)]
    pub fn pden(&self) -> PDEN_R {
        PDEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Custom time for tCLK"]
    #[inline(always)]
    pub fn tclkprepen(&self) -> TCLKPREPEN_R {
        TCLKPREPEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Custom time for tCLK"]
    #[inline(always)]
    pub fn tclkzeroen(&self) -> TCLKZEROEN_R {
        TCLKZEROEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Custom time for tHS"]
    #[inline(always)]
    pub fn thsprepen(&self) -> THSPREPEN_R {
        THSPREPEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Custom time for tHS"]
    #[inline(always)]
    pub fn thstrailen(&self) -> THSTRAILEN_R {
        THSTRAILEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Custom time for tHS"]
    #[inline(always)]
    pub fn thszeroen(&self) -> THSZEROEN_R {
        THSZEROEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Custom time for tLPX for data lanes enable"]
    #[inline(always)]
    pub fn tlpxden(&self) -> TLPXDEN_R {
        TLPXDEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Custom time for tHS"]
    #[inline(always)]
    pub fn thsexiten(&self) -> THSEXITEN_R {
        THSEXITEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Custom time for tLPX for clock lane enable"]
    #[inline(always)]
    pub fn tlpxcen(&self) -> TLPXCEN_R {
        TLPXCEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Custom time for tCLK"]
    #[inline(always)]
    pub fn tclkposten(&self) -> TCLKPOSTEN_R {
        TCLKPOSTEN_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Unit interval multiplied by 4"]
    #[inline(always)]
    #[must_use]
    pub fn uix4(&mut self) -> UIX4_W<0> {
        UIX4_W::new(self)
    }
    #[doc = "Bit 6 - Swap clock lane pins"]
    #[inline(always)]
    #[must_use]
    pub fn swcl(&mut self) -> SWCL_W<6> {
        SWCL_W::new(self)
    }
    #[doc = "Bit 7 - Swap data lane 0 pins"]
    #[inline(always)]
    #[must_use]
    pub fn swdl0(&mut self) -> SWDL0_W<7> {
        SWDL0_W::new(self)
    }
    #[doc = "Bit 8 - Swap data lane 1 pins"]
    #[inline(always)]
    #[must_use]
    pub fn swdl1(&mut self) -> SWDL1_W<8> {
        SWDL1_W::new(self)
    }
    #[doc = "Bit 9 - Invert high"]
    #[inline(always)]
    #[must_use]
    pub fn hsicl(&mut self) -> HSICL_W<9> {
        HSICL_W::new(self)
    }
    #[doc = "Bit 10 - Invert the high-speed data signal on data lane 0"]
    #[inline(always)]
    #[must_use]
    pub fn hsidl0(&mut self) -> HSIDL0_W<10> {
        HSIDL0_W::new(self)
    }
    #[doc = "Bit 11 - Invert the high-speed data signal on data lane 1"]
    #[inline(always)]
    #[must_use]
    pub fn hsidl1(&mut self) -> HSIDL1_W<11> {
        HSIDL1_W::new(self)
    }
    #[doc = "Bit 12 - Force in TX Stop mode the clock lane"]
    #[inline(always)]
    #[must_use]
    pub fn ftxsmcl(&mut self) -> FTXSMCL_W<12> {
        FTXSMCL_W::new(self)
    }
    #[doc = "Bit 13 - Force in TX Stop mode the data lanes"]
    #[inline(always)]
    #[must_use]
    pub fn ftxsmdl(&mut self) -> FTXSMDL_W<13> {
        FTXSMDL_W::new(self)
    }
    #[doc = "Bit 14 - Contention detection OFF on data lanes"]
    #[inline(always)]
    #[must_use]
    pub fn cdoffdl(&mut self) -> CDOFFDL_W<14> {
        CDOFFDL_W::new(self)
    }
    #[doc = "Bit 16 - Turn disable data lanes"]
    #[inline(always)]
    #[must_use]
    pub fn tddl(&mut self) -> TDDL_W<16> {
        TDDL_W::new(self)
    }
    #[doc = "Bit 18 - Pull-down enable"]
    #[inline(always)]
    #[must_use]
    pub fn pden(&mut self) -> PDEN_W<18> {
        PDEN_W::new(self)
    }
    #[doc = "Bit 19 - Custom time for tCLK"]
    #[inline(always)]
    #[must_use]
    pub fn tclkprepen(&mut self) -> TCLKPREPEN_W<19> {
        TCLKPREPEN_W::new(self)
    }
    #[doc = "Bit 20 - Custom time for tCLK"]
    #[inline(always)]
    #[must_use]
    pub fn tclkzeroen(&mut self) -> TCLKZEROEN_W<20> {
        TCLKZEROEN_W::new(self)
    }
    #[doc = "Bit 21 - Custom time for tHS"]
    #[inline(always)]
    #[must_use]
    pub fn thsprepen(&mut self) -> THSPREPEN_W<21> {
        THSPREPEN_W::new(self)
    }
    #[doc = "Bit 22 - Custom time for tHS"]
    #[inline(always)]
    #[must_use]
    pub fn thstrailen(&mut self) -> THSTRAILEN_W<22> {
        THSTRAILEN_W::new(self)
    }
    #[doc = "Bit 23 - Custom time for tHS"]
    #[inline(always)]
    #[must_use]
    pub fn thszeroen(&mut self) -> THSZEROEN_W<23> {
        THSZEROEN_W::new(self)
    }
    #[doc = "Bit 24 - Custom time for tLPX for data lanes enable"]
    #[inline(always)]
    #[must_use]
    pub fn tlpxden(&mut self) -> TLPXDEN_W<24> {
        TLPXDEN_W::new(self)
    }
    #[doc = "Bit 25 - Custom time for tHS"]
    #[inline(always)]
    #[must_use]
    pub fn thsexiten(&mut self) -> THSEXITEN_W<25> {
        THSEXITEN_W::new(self)
    }
    #[doc = "Bit 26 - Custom time for tLPX for clock lane enable"]
    #[inline(always)]
    #[must_use]
    pub fn tlpxcen(&mut self) -> TLPXCEN_W<26> {
        TLPXCEN_W::new(self)
    }
    #[doc = "Bit 27 - Custom time for tCLK"]
    #[inline(always)]
    #[must_use]
    pub fn tclkposten(&mut self) -> TCLKPOSTEN_W<27> {
        TCLKPOSTEN_W::new(self)
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
